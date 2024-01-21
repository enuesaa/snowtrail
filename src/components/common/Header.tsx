import { IoMdSnow } from 'react-icons/io'
import styles from './Header.css'
import { Box, Flex, Link, Separator } from '@radix-ui/themes'

export const Header = () => {
  return (
    <>
      <header className={styles.top}>
        <Flex>
          <Box grow='1' shrink='0' ml='5'>
            <Link href='/'><IoMdSnow />snowtrail</Link>
          </Box>
          {/* <Box grow='0' shrink='0' style={{width: '35px'}}>
            <Link href='/settings'><MdOutlineSettings /></Link>
          </Box> */}
        </Flex>
      </header>
      <Separator size='4' mb='2' />
    </>
  )
}
