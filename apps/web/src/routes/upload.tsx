import * as React from 'react'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import Text, { textVariants } from '@/components/ui/text'
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
import { useForm } from '@tanstack/react-form'
import { z } from 'zod'
import {
  Field,
  FieldError,
  FieldGroup,
  FieldLabel,
} from '@/components/ui/field'
import { Input } from '@/components/ui/input'
import {
  Combobox,
  ComboboxChip,
  ComboboxChips,
  ComboboxChipsInput,
  ComboboxContent,
  ComboboxEmpty,
  ComboboxItem,
  ComboboxList,
  ComboboxValue,
  useComboboxAnchor,
} from '@/components/ui/combobox'

export const Route = createFileRoute('/upload')({
  component: UploadPage,
})

const uploadVideoFormSchema = z.object({
  title: z
    .string()
    .nonempty('Title must be not empty')
    .min(5, 'Title must be at least 5 characters')
    .max(100, 'Title must be at most 40 characters'),
  description: z
    .string()
    .min(5, 'Title must be at least 5 characters')
    .max(400, 'Title must be at most 40 characters'),
  categories: z
    .array(z.string())
    .min(1, 'Categories must be at least 1 categories')
    .max(2, 'Categories must be at most 10 categories'),
  visibility: z.enum(['public', 'private']),
})

function UploadPage() {
  const anchor = useComboboxAnchor()

  const [files, setFiles] = React.useState<File[]>([])
  const form = useForm({
    defaultValues: {
      title: '',
      description: '',
      categories: [] as string[],
      visibility: 'public',
    },
    validators: {
      onSubmit: uploadVideoFormSchema,
      onChange: uploadVideoFormSchema,
    },
  })

  return (
    <>
      <section className="flex justify-center items-center min-h-[90dvh] ">
        <Card className="w-xl">
          <CardHeader>
            <CardTitle className={textVariants({ tag: 'h4' })}>
              Upload Video
            </CardTitle>
            <CardDescription>Share your video</CardDescription>
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
                    <Text tag="p">Drop your video here or click to select</Text>
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
            <form
              id="upload-video-form"
              onSubmit={(e) => {
                e.preventDefault()
                form.handleSubmit()
              }}
              className="py-4"
            >
              <FieldGroup>
                <form.Field
                  name="title"
                  children={(field) => {
                    const isInvalid =
                      field.state.meta.isTouched && !field.state.meta.isValid
                    return (
                      <Field data-invalid={isInvalid}>
                        <FieldLabel htmlFor={field.name}>Title</FieldLabel>
                        <Input
                          id={field.name}
                          name={field.name}
                          value={field.state.value}
                          onBlur={field.handleBlur}
                          onChange={(e) => field.handleChange(e.target.value)}
                          aria-invalid={isInvalid}
                          placeholder="Enter video title"
                          autoComplete="off"
                        />
                        {isInvalid && (
                          <FieldError errors={field.state.meta.errors} />
                        )}
                      </Field>
                    )
                  }}
                />
                <form.Field
                  name="description"
                  children={(field) => {
                    const isInvalid =
                      field.state.meta.isTouched && !field.state.meta.isValid
                    return (
                      <Field data-invalid={isInvalid}>
                        <FieldLabel htmlFor={field.name}>
                          Description
                        </FieldLabel>
                        <Input
                          id={field.name}
                          name={field.name}
                          value={field.state.value}
                          onBlur={field.handleBlur}
                          onChange={(e) => field.handleChange(e.target.value)}
                          aria-invalid={isInvalid}
                          placeholder="Describe your video"
                          autoComplete="off"
                        />
                        {isInvalid && (
                          <FieldError errors={field.state.meta.errors} />
                        )}
                      </Field>
                    )
                  }}
                />
                <form.Field
                  name="categories"
                  children={(field) => {
                    const isInvalid =
                      field.state.meta.isTouched && !field.state.meta.isValid
                    return (
                      <Field data-invalid={isInvalid}>
                        <FieldLabel htmlFor={field.name}>Categories</FieldLabel>
                        <Combobox
                          multiple
                          autoHighlight
                          items={[
                            'horror',
                            'scifi',
                            'fantasy',
                            'action',
                            'adventure',
                            'romance',
                            'comedy',
                          ]}
                          value={field.state.value}
                          onValueChange={field.handleChange}
                          defaultValue={field.state.value}
                        >
                          <ComboboxChips
                            ref={anchor}
                            aria-invalid={isInvalid}
                            className="w-full"
                          >
                            <ComboboxValue>
                              {(values) => (
                                <>
                                  {values.map((value: string) => (
                                    <ComboboxChip key={value}>
                                      {value}
                                    </ComboboxChip>
                                  ))}
                                  <ComboboxChipsInput
                                    placeholder="Select video categories"
                                    aria-invalid={isInvalid}
                                  />
                                </>
                              )}
                            </ComboboxValue>
                          </ComboboxChips>
                          <ComboboxContent anchor={anchor}>
                            <ComboboxEmpty>No categories found.</ComboboxEmpty>
                            <ComboboxList>
                              {(item) => (
                                <ComboboxItem key={item} value={item}>
                                  {item}
                                </ComboboxItem>
                              )}
                            </ComboboxList>
                          </ComboboxContent>
                        </Combobox>
                        {isInvalid && (
                          <FieldError errors={field.state.meta.errors} />
                        )}
                      </Field>
                    )
                  }}
                />
              </FieldGroup>
            </form>
          </CardContent>
        </Card>
      </section>
    </>
  )
}
