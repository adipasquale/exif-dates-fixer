import "./App.css"

import { useEffect, useState, useLayoutEffect } from "react"
import { invoke } from "@tauri-apps/api/tauri"
import { open } from '@tauri-apps/api/dialog'
import { exists, BaseDirectory, readTextFile, writeTextFile } from '@tauri-apps/api/fs'
import { HotkeysProvider, useHotkeys } from "react-hotkeys-hook"

import Sidebar from "./Sidebar"
import PhotosCollection from "./PhotosCollection"

function App() {
  const [dirPath, setDirPath] = useState("")
  const [selectedFileInfo, setSelectedFileInfo] = useState(null)
  const [newDate, onChangeNewDate] = useState("2020-03-04")

  const [fileInfosRaw, setFilesInfosRaw] = useState([])
  const fileInfos = fileInfosRaw
    .map((fi) => ({ ...fi, dateParsed: new Date(fi.date) }))
    .sort((a, b) => b.dateParsed - a.dateParsed)
    .map((fi, index) => ({ ...fi, index }))

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
      setFilesInfosRaw(await invoke("listdir", { dirpath: dirPath }))
    }
    init()
  }, [dirPath])

  function selectAndScrollTo(fileInfo) {
    setSelectedFileInfo(fileInfo)
    setTimeout(() => {
      const elt = document.querySelector(`.photo[data-path="${fileInfo.path}"]`)
      const rect = elt.getBoundingClientRect()
      if (rect.top > 0 && rect.bottom < window.innerHeight) return
      elt.scrollIntoView()
    }, 50)
  }

  useHotkeys('j, left', () => {
    if (!selectedFileInfo || selectedFileInfo.index <= 0) return
    selectAndScrollTo(fileInfos[selectedFileInfo.index - 1])
  })

  useHotkeys('k, right', () => {
    if (!selectedFileInfo || selectedFileInfo.index >= fileInfos.length - 1) return
    selectAndScrollTo(fileInfos[selectedFileInfo.index + 1])
  })

  function getColumnsPerRow() {
    const grid = document.querySelector(".photosGrid")
    if (!grid) return 0
    const gridComputedStyle = window.getComputedStyle(grid)
    return gridComputedStyle.getPropertyValue("grid-template-columns").split(" ").length
  }

  useHotkeys('down', (e) => {
    e.preventDefault()
    if (!selectedFileInfo) return
    const newIndex = selectedFileInfo.index + getColumnsPerRow()
    if (newIndex >= fileInfos.length) return
    selectAndScrollTo(fileInfos[newIndex])
  })

  useHotkeys('up', (e) => {
    e.preventDefault()
    if (!selectedFileInfo) return
    const newIndex = selectedFileInfo.index - getColumnsPerRow()
    if (newIndex < 0) return
    selectAndScrollTo(fileInfos[newIndex])
  })

  useHotkeys('d', async () => {
    if (!selectedFileInfo) return
    await invoke("rmphoto", { path: selectedFileInfo.path })
    setSelectedFileInfo(fileInfos[selectedFileInfo.index + 1])
    setFilesInfosRaw(fileInfos.filter(fi => fi.path !== selectedFileInfo.path))
  })


  async function onSubmitNewDate() {
    let fileInfo = await invoke("set_date", { path: selectedFileInfo.path, date: newDate })
    fileInfo.dateParsed = new Date(fileInfo.date)
    setFilesInfosRaw([fileInfo, ...fileInfos.filter(fi => fi.path !== fileInfo.path)])
    selectAndScrollTo(fileInfo)
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
