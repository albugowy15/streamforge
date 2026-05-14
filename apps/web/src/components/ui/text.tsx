import * as React from 'react'
import { cva } from 'class-variance-authority'
import { cn } from '@/lib/utils'

export const textVariants = cva('', {
  variants: {
    tag: {
      h1: 'text-4xl font-extrabold tracking-tight text-balance',
      h2: 'pb-2 text-3xl font-semibold tracking-tight first:mt-0',
      h3: 'text-2xl font-semibold tracking-tight',
      h4: 'text-xl font-semibold tracking-tight',
      p: 'leading-7',
      small: 'text-sm leading-none font-medium',
    },
  },
  defaultVariants: {
    tag: 'p',
  },
})

type TextTag = 'h1' | 'h2' | 'h3' | 'h4' | 'p' | 'small'

type TextProps = React.ComponentPropsWithoutRef<TextTag> & {
  tag?: TextTag
}

export default function Text({ tag = 'p', className, ...props }: TextProps) {
  const Component = tag as React.ElementType
  return (
    <Component className={cn(textVariants({ tag }), className)} {...props} />
  )
}
