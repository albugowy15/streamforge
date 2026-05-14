'use client'

import * as React from 'react'
import { XIcon } from 'lucide-react'

import { cn } from '@/lib/utils'
import { Button } from '@/components/ui/button'

interface FileUploaderContextValue {
  files: File[]
  setFiles: React.Dispatch<React.SetStateAction<File[]>>
  isDragging: boolean
  setIsDragging: React.Dispatch<React.SetStateAction<boolean>>
  onValueChange?: (files: File[]) => void
  maxFiles?: number
  maxSize?: number
  accept?: Record<string, string[]>
  disabled?: boolean
}

const FileUploaderContext =
  React.createContext<FileUploaderContextValue | null>(null)

function useFileUploader() {
  const context = React.useContext(FileUploaderContext)
  if (!context) {
    throw new Error('useFileUploader must be used within a FileUploader')
  }
  return context
}

interface FileUploaderProps {
  value?: File[]
  onValueChange?: (files: File[]) => void
  maxFiles?: number
  maxSize?: number
  accept?: Record<string, string[]>
  disabled?: boolean
  children: React.ReactNode
  className?: string
}

function FileUploader({
  value: valueProp,
  onValueChange,
  maxFiles,
  maxSize,
  accept,
  disabled,
  children,
  className,
}: FileUploaderProps) {
  const [internalFiles, setInternalFiles] = React.useState<File[]>([])
  const [isDragging, setIsDragging] = React.useState(false)

  const files = valueProp ?? internalFiles
  const setFiles = React.useCallback(
    (newFiles: React.SetStateAction<File[]>) => {
      const updatedFiles =
        typeof newFiles === 'function' ? newFiles(files) : newFiles
      if (!valueProp) {
        setInternalFiles(updatedFiles)
      }
      onValueChange?.(updatedFiles)
    },
    [files, onValueChange, valueProp],
  )

  return (
    <FileUploaderContext.Provider
      value={{
        files,
        setFiles,
        isDragging,
        setIsDragging,
        onValueChange,
        maxFiles,
        maxSize,
        accept,
        disabled,
      }}
    >
      <div data-slot="file-uploader" className={cn('grid gap-2', className)}>
        {children}
      </div>
    </FileUploaderContext.Provider>
  )
}

function FileUploaderTrigger({
  className,
  children,
  ...props
}: React.ComponentProps<'div'>) {
  const { setFiles, setIsDragging, isDragging, disabled, maxFiles, accept } =
    useFileUploader()
  const inputRef = React.useRef<HTMLInputElement>(null)

  const onDrop = React.useCallback(
    (e: React.DragEvent) => {
      e.preventDefault()
      e.stopPropagation()
      setIsDragging(false)

      if (disabled) return

      const droppedFiles = Array.from(e.dataTransfer.files)
      if (droppedFiles.length > 0) {
        setFiles((prev) => {
          const combined = [...prev, ...droppedFiles]
          return maxFiles ? combined.slice(0, maxFiles) : combined
        })
      }
    },
    [disabled, maxFiles, setFiles, setIsDragging],
  )

  const onDragOver = React.useCallback(
    (e: React.DragEvent) => {
      e.preventDefault()
      e.stopPropagation()
      if (!disabled) setIsDragging(true)
    },
    [disabled, setIsDragging],
  )

  const onDragLeave = React.useCallback(
    (e: React.DragEvent) => {
      e.preventDefault()
      e.stopPropagation()
      setIsDragging(false)
    },
    [setIsDragging],
  )

  const onInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files.length > 0) {
      const selectedFiles = Array.from(e.target.files)
      setFiles((prev) => {
        const combined = [...prev, ...selectedFiles]
        return maxFiles ? combined.slice(0, maxFiles) : combined
      })
    }
  }

  const handleClick = () => {
    if (!disabled) {
      inputRef.current?.click()
    }
  }

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault()
      handleClick()
    }
  }

  const acceptString = accept
    ? Object.entries(accept)
        .map(([mime, ext]) => [mime, ...ext])
        .flat()
        .join(',')
    : undefined

  return (
    <div
      data-slot="file-uploader-trigger"
      role="button"
      tabIndex={disabled ? -1 : 0}
      data-dragging={isDragging}
      data-disabled={disabled}
      className={cn(
        'relative flex cursor-pointer flex-col items-center justify-center gap-2 rounded-lg border-2 border-dashed border-muted-foreground/25 bg-background p-6 transition-colors hover:bg-muted/50 focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50 outline-none data-[dragging=true]:border-primary data-[dragging=true]:bg-primary/5 data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50',
        className,
      )}
      onDrop={onDrop}
      onDragOver={onDragOver}
      onDragLeave={onDragLeave}
      onClick={handleClick}
      onKeyDown={handleKeyDown}
      {...props}
    >
      <input
        ref={inputRef}
        type="file"
        multiple={(maxFiles ?? 2) > 1}
        accept={acceptString}
        className="sr-only"
        onChange={onInputChange}
        disabled={disabled}
      />
      {children}
    </div>
  )
}

function FileUploaderContent({
  className,
  ...props
}: React.ComponentProps<'div'>) {
  return (
    <div
      data-slot="file-uploader-content"
      className={cn(
        'flex flex-col items-center justify-center gap-1',
        className,
      )}
      {...props}
    />
  )
}

function FileUploaderList({ className, ...props }: React.ComponentProps<'ul'>) {
  const { files } = useFileUploader()

  if (files.length === 0) return null

  return (
    <ul
      data-slot="file-uploader-list"
      className={cn('grid gap-2', className)}
      {...props}
    />
  )
}

interface FileUploaderItemProps extends React.ComponentProps<'li'> {
  index: number
}

const FileUploaderItemContext = React.createContext<{
  file: File
  index: number
} | null>(null)

function useFileUploaderItem() {
  const context = React.useContext(FileUploaderItemContext)
  if (!context) {
    throw new Error(
      'useFileUploaderItem must be used within a FileUploaderItem',
    )
  }
  return context
}

function FileUploaderItem({
  index,
  className,
  children,
  ...props
}: FileUploaderItemProps) {
  const { files } = useFileUploader()
  const file = files[index]

  return (
    <FileUploaderItemContext.Provider value={{ file, index }}>
      <li
        data-slot="file-uploader-item"
        className={cn(
          'flex items-center gap-3 rounded-lg border border-border p-2 pr-1',
          className,
        )}
        {...props}
      >
        {children}
      </li>
    </FileUploaderItemContext.Provider>
  )
}

function FileUploaderItemName({
  className,
  ...props
}: React.ComponentProps<'span'>) {
  const { file } = useFileUploaderItem()
  return (
    <span
      data-slot="file-uploader-item-name"
      className={cn('flex-1 truncate text-sm font-medium', className)}
      {...props}
    >
      {file.name}
    </span>
  )
}

function FileUploaderItemSize({
  className,
  ...props
}: React.ComponentProps<'span'>) {
  const { file } = useFileUploaderItem()

  const formatSize = (bytes: number) => {
    if (bytes === 0) return '0 B'
    const k = 1024
    const sizes = ['B', 'KB', 'MB', 'GB']
    const i = Math.floor(Math.log(bytes) / Math.log(k))
    return `${parseFloat((bytes / Math.pow(k, i)).toFixed(1))} ${sizes[i]}`
  }

  return (
    <span
      data-slot="file-uploader-item-size"
      className={cn('text-xs text-muted-foreground', className)}
      {...props}
    >
      {formatSize(file.size)}
    </span>
  )
}

function FileUploaderItemRemove({
  className,
  ...props
}: React.ComponentProps<typeof Button>) {
  const { setFiles } = useFileUploader()
  const { index } = useFileUploaderItem()

  return (
    <Button
      variant="ghost"
      size="icon-xs"
      data-slot="file-uploader-item-remove"
      className={cn('text-muted-foreground hover:text-foreground', className)}
      onClick={(e) => {
        e.stopPropagation()
        setFiles((prev) => prev.filter((_, i) => i !== index))
      }}
      {...props}
    >
      <XIcon />
    </Button>
  )
}

export {
  FileUploader,
  FileUploaderTrigger,
  FileUploaderContent,
  FileUploaderList,
  FileUploaderItem,
  FileUploaderItemName,
  FileUploaderItemSize,
  FileUploaderItemRemove,
}
