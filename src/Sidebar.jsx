import "./Sidebar.css"
import { convertFileSrc } from '@tauri-apps/api/tauri'

export default function Sidebar({ fileInfo, newDate, onNewDateChange, onSubmit }) {
  return (
    <div className="sidebar">
      {fileInfo && <div>
        <div>
          <img src={convertFileSrc(fileInfo.path)} />
        </div>
        <div style={{ textAlign: "center" }}>
          {fileInfo.date ?
            <b>{fileInfo.date}</b>
            :
            <span style={{ color: "red" }}>original date missing</span>
          }
        </div>
        <div>
          <input type="date" value={newDate} onChange={(e) => onNewDateChange(e.target.value)} />
          <button type="submit" onClick={onSubmit}>
            Save
          </button>
        </div>
        <ul>
          <li>index: {fileInfo.index}</li>
          <li>filename : {fileInfo.filename}</li>
          {Object.entries(fileInfo.exifDateTags).map(([tagName, tagValue]) =>
            <li key={tagName}>{tagName} : {tagValue}</li>
          )}
        </ul>
      </div>
      }
    </div>
  )
}
