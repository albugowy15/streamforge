import * as React from 'react'

const previewCache = new WeakMap<File, Promise<string>>()

const getPreview = (file: File) => {
  let promise = previewCache.get(file)
  if (!promise) {
    promise = new Promise((resolve) => {
      const reader = new FileReader()
      reader.onloadend = () => resolve(reader.result as string)
      reader.readAsDataURL(file)
    })
    previewCache.set(file, promise)
  }
  return promise
}

interface ImagePreviewProps {
  file: File
  className?: string
}

export function ImagePreview({ file, className }: ImagePreviewProps) {
  const preview = React.use(getPreview(file))

  return <img src={preview} alt={file.name} className={className} />
}
