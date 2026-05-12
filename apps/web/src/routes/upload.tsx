import * as React from 'react'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { createFileRoute } from '@tanstack/react-router'
import { UploadIcon } from 'lucide-react'

import {
  FileUploader,
  FileUploaderContent,
  FileUploaderItem,
  FileUploaderItemName,
  FileUploaderItemRemove,
  FileUploaderItemSize,
  FileUploaderList,
  FileUploaderTrigger,
} from '@/components/ui/file-uploader'

export const Route = createFileRoute('/upload')({
  component: UploadPage,
})

function UploadPage() {
  const [files, setFiles] = React.useState<File[]>([])

  return (
    <div className="flex min-h-svh flex-col items-center justify-center bg-muted p-6 md:p-10">
      <div className="w-full max-w-sm md:max-w-4xl">
        <div className="flex flex-col gap-6">
          <div className="flex flex-col gap-2">
            <h1 className="text-4xl font-bold text-foreground">Upload Video</h1>
            <p className="text-muted-foreground">
              Share your content with the world
            </p>
          </div>
          <Card className="overflow-hidden">
            <CardHeader>
              <CardTitle>Choose a video to upload</CardTitle>
              <CardDescription>
                Drag and drop your video file here or click to browse
              </CardDescription>
            </CardHeader>
            <CardContent>
              <FileUploader
                value={files}
                onValueChange={setFiles}
                maxFiles={1}
                accept={{ 'video/*': ['.mp4', '.mkv', '.mov'] }}
              >
                {files.length === 0 && (
                  <FileUploaderTrigger>
                    <FileUploaderContent>
                      <UploadIcon className="size-12 text-muted-foreground transition-colors group-hover:text-accent mb-3" />
                      <p className="text-sm text-muted-foreground transition-colors group-hover:text-foreground">
                        Drop your video here or click to select
                      </p>
                    </FileUploaderContent>
                  </FileUploaderTrigger>
                )}

                <FileUploaderList>
                  {files.map((_, i) => (
                    <FileUploaderItem key={i} index={i}>
                      <FileUploaderItemName />
                      <FileUploaderItemSize />
                      <FileUploaderItemRemove />
                    </FileUploaderItem>
                  ))}
                </FileUploaderList>
              </FileUploader>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  )
}
