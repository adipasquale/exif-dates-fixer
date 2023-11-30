import { useEffect, useState } from "react"
import { invoke } from "@tauri-apps/api/tauri"
import { open } from '@tauri-apps/api/dialog'
import { exists, BaseDirectory, readTextFile, writeTextFile } from '@tauri-apps/api/fs'
import "./App.css"
import { HotkeysProvider, useHotkeys } from "react-hotkeys-hook"

import Sidebar from "./Sidebar"
import PhotosCollection from "./photosCollection"

function App() {
  const [dirPath, setDirPath] = useState("")
  const [fileInfos, setFilesInfos] = useState([])
  const [selectedFileInfo, setSelectedFileInfo] = useState(null)
  const [newDate, onChangeNewDate] = useState("2020-03-04")

  async function selectDirPath() {
    const newDirPath = await open({ directory: true })
    writeTextFile('dirPath.conf', newDirPath, { dir: BaseDirectory.AppConfig })
    setDirPath(newDirPath)
  }

  useEffect(() => {
    async function init() {
      const dirPathFileExists = await exists('dirPath.conf', { dir: BaseDirectory.AppConfig })
      if (!dirPathFileExists) return

      const dirPath = await readTextFile('dirPath.conf', { dir: BaseDirectory.AppConfig })
      if (!dirPath) return
      setDirPath(dirPath)
    }
    init()
  }, [])


  useEffect(() => {
    async function init() {
      setFilesInfos(
        (await invoke("listdir", { dirpath: dirPath }))
          .map((fi) => ({ ...fi, dateParsed: new Date(fi.date) }))
          .sort((a, b) => b.dateParsed - a.dateParsed)
          .map((fi, index) => ({ ...fi, index }))
      )
    }
    init()
  }, [dirPath])

  useHotkeys('j', () => {
    if (!selectedFileInfo || selectedFileInfo.index <= 0) return
    setSelectedFileInfo(fileInfos[selectedFileInfo.index - 1])
  })

  useHotkeys('k', () => {
    if (!selectedFileInfo || selectedFileInfo.index >= fileInfos.length - 1) return
    setSelectedFileInfo(fileInfos[selectedFileInfo.index + 1])
  })

  useHotkeys('d', async () => {
    if (!selectedFileInfo) return
    await invoke("rmphoto", { path: selectedFileInfo.path })
    setSelectedFileInfo(fileInfos[selectedFileInfo.index + 1])
    setFilesInfos(fileInfos.filter(fi => fi.path !== selectedFileInfo.path))
  })


  async function onSubmitNewDate() {
    let fileInfo = await invoke("set_date", { path: selectedFileInfo.path, date: newDate })
    fileInfo.dateParsed = new Date(fileInfo.date)
    const index = fileInfos.findIndex(fi => fi.path === selectedFileInfo.path)
    let newFileInfos = fileInfos.slice()
    newFileInfos[index] = fileInfo
    newFileInfos.sort((a, b) => b.dateParsed - a.dateParsed)
    newFileInfos = newFileInfos.map((fi, index) => ({ ...fi, index }))
    setFilesInfos(newFileInfos)
    setSelectedFileInfo(newFileInfos.find(fi => fi.path === fileInfo.path))
    setTimeout(() => {
      document.querySelector(`.photo[data-path="${fileInfo.path}"]`).scrollIntoView()
    }, 50)
  }

  return (
    <HotkeysProvider>
      <div className="container">
        <main>
          <h1>Exif Dates Fixer</h1>

          <div className="row">
            <button type="submit" onClick={() => { selectDirPath() }}>
              Choose Directory
            </button>
          </div>

          <p>current directory : {dirPath}</p>

          <PhotosCollection fileInfos={fileInfos} selectedFileInfo={selectedFileInfo} setSelectedFileInfo={setSelectedFileInfo} />
        </main >

        <Sidebar fileInfo={selectedFileInfo} newDate={newDate} onSubmit={onSubmitNewDate} onNewDateChange={onChangeNewDate} />
      </div >
    </HotkeysProvider>
  )
}

export default App
