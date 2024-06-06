import Home from '@/components/icons/home'
import Workflows from '@/components/icons/workflows'
import Settings from '@/components/icons/settings'


export const menuOptions = [
    { name: 'Dashboard', Component: Home, href: '/dashboard' },
    { name: 'Workflows', Component: Workflows, href: '/workflows' },
    { name: 'Settings', Component: Settings, href: '/settings' },
    // { name: 'Connections', Component: Category, href: '/connections' },
]