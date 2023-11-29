import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from '@tauri-apps/api/dialog';
import { exists, BaseDirectory, readTextFile, writeTextFile } from '@tauri-apps/api/fs';
import "./App.css";
import { convertFileSrc } from '@tauri-apps/api/tauri';

function App() {
  const [dirPath, setDirPath] = useState("");
  const [fileInfos, setFilesInfos] = useState(null);
  const [fileInfosUngrouped, setFilesInfosUngrouped] = useState([]);
  const [selectedFileInfo, setSelectedFileInfo] = useState(null);

  async function selectDirPath() {
    const newDirPath = await open({
      directory: true
    })
    writeTextFile('dirPath.conf', newDirPath, { dir: BaseDirectory.AppConfig })
    setDirPath(newDirPath)
  }

  useEffect(() => {
    async function init() {
      const dirPathFileExists = await exists('dirPath.conf', { dir: BaseDirectory.AppConfig });
      if (!dirPathFileExists) return

      const dirPath = await readTextFile('dirPath.conf', { dir: BaseDirectory.AppConfig })
      if (!dirPath) return
      setDirPath(dirPath)
    }
    init()
  }, [])

  useEffect(() => {
    setFilesInfos(null)
    setFilesInfos(
      fileInfosUngrouped
        .reduce((acc, fi) => {
          const { year, week } = fi
          if (!acc[year]) acc[year] = {}
          if (!acc[year][week]) acc[year][week] = []
          acc[year][week].push(fi)
          return acc
        }, {})
    )
  }, [fileInfosUngrouped])

  useEffect(() => {
    async function init() {
      setFilesInfosUngrouped(
        (await invoke("listdir", { dirpath: dirPath }))
          .map(fileInfo => ({
            ...fileInfo,
            srcPath: convertFileSrc(fileInfo.path),
          }))
          .sort((a, b) => a.date - b.date)
      )
    }
    init()
  }, [dirPath])

  useEffect(() => {
    const handleKeyDown = async (e) => {
      if (!selectedFileInfo) return;
      const selectedFileInfoIndex = fileInfosUngrouped.findIndex(fi => fi.path === selectedFileInfo.path)
      if (selectedFileInfoIndex < 0) {
        console.error("selectedFileInfo not found in fileInfosUngrouped")
        return null
      }
      switch (e.keyCode) {
        case 74: // letter J
          if (selectedFileInfo && selectedFileInfoIndex > 0) {
            setSelectedFileInfo(fileInfosUngrouped[selectedFileInfoIndex - 1])
          }
          break;
        case 75: // letter K
          if (selectedFileInfo && selectedFileInfoIndex < fileInfosUngrouped.length - 1) {
            setSelectedFileInfo(fileInfosUngrouped[selectedFileInfoIndex + 1])
          }
          break;
        case 68: // letter D
          if (selectedFileInfo) {
            await invoke("rmphoto", { path: selectedFileInfo.path })
            setSelectedFileInfo(fileInfosUngrouped[selectedFileInfoIndex + 1])
            setFilesInfosUngrouped(fileInfosUngrouped.filter(fi => fi.path !== selectedFileInfo.path))
          }
          break;

      }
    }
    document.addEventListener('keydown', handleKeyDown);

    return function cleanup() {
      document.removeEventListener('keydown', handleKeyDown);
    }
  }, [selectedFileInfo]);


  return (
    <div className="container">
      <main>
        <h1>Exif Dates Fixer</h1>

        <div className="row">
          <button type="submit" onClick={() => { selectDirPath(); }}>
            Choose Directory
          </button>
        </div>

        <p>current directory : {dirPath}</p>

        {fileInfos && Object.keys(fileInfos).sort().reverse().map(year => {
          return <div key={year}>
            <h2>{year}</h2>
            {Object.keys(fileInfos[year]).sort((a, b) => a - b).reverse().map(week => {
              return <div key={week}>
                <h3>Week {week}</h3>
                <div className="photosGrid">
                  {fileInfos[year][week].map((fileInfo, index) => {
                    const selected = selectedFileInfo && selectedFileInfo.path === fileInfo.path
                    return <div key={index} onClick={() => setSelectedFileInfo(fileInfo)} className={["photo", selected ? "selected" : ""].join(" ")}>
                      <img src={fileInfo.srcPath} />
                    </div>
                  })}
                </div>
              </div>
            })}
          </div>
        })}
      </main >

      <div className="sidebar">
        {selectedFileInfo && <div>
          <div>
            <img src={selectedFileInfo.srcPath} />
          </div>
          <div style={{ textAlign: "center" }}>
            {selectedFileInfo.date ?
              <b>{selectedFileInfo.date}</b>
              :
              <span style={{ color: "red" }}>original date missing</span>
            }
          </div>
          <ul>
            <li>filename : {selectedFileInfo.filename}</li>
            {Object.entries(selectedFileInfo.exifDateTags).map(([tagName, tagValue]) =>
              <li key={tagName}>{tagName} : {tagValue}</li>
            )}
          </ul>
        </div>
        }
      </div>
    </div >
  );
}

export default App;
