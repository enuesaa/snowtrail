import { Link as WouterLink } from 'wouter'
import { Link as RadixLink } from '@radix-ui/themes'
import { type LinkProps } from 'node_modules/@radix-ui/themes/dist/esm/components/link'

export const Link = ({ href, children, ...props }: LinkProps) => {
  return (
    <RadixLink asChild {...props}>
      <WouterLink to={href ?? '/'}>{children}</WouterLink>
    </RadixLink>
  )
}
