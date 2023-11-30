import "./PhotosCollection.css"
import { convertFileSrc } from '@tauri-apps/api/tauri'

export default function PhotosCollection({ fileInfos, selectedFileInfo, setSelectedFileInfo }) {

  if (!fileInfos) return null

  let groups = fileInfos
    .reduce((acc, fi) => {
      const { year, week } = fi
      if (!acc[year]) acc[year] = {}
      if (!acc[year][week]) acc[year][week] = []
      acc[year][week].push(fi)
      return acc
    }, {})

  let years = Object.keys(groups).sort().reverse()

  return (
    <div>
      {years.map(year => {
        return <div key={year}>
          <h2>{year}</h2>
          {Object.keys(groups[year]).sort((a, b) => a - b).reverse().map(week => {
            return <div key={week}>
              <h3>Week {week}</h3>
              <div className="photosGrid">
                {groups[year][week].map((fileInfo, index) => {
                  const selected = selectedFileInfo && selectedFileInfo.path === fileInfo.path
                  return <div
                    key={index}
                    data-path={fileInfo.path}
                    onClick={() => setSelectedFileInfo(fileInfo)}
                    className={["photo", selected ? "selected" : ""].join(" ")}
                  >
                    <img src={convertFileSrc(fileInfo.path)} />
                  </div>
                })}
              </div>
            </div>
          })}
        </div>
      })}
    </div>
  )
}
