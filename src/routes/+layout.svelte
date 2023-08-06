<script>
    import {
        AppShell,
        Button,
        Center,
        Flex,
        Footer,
        Group,
        Header,
        Title,
    } from '@svelteuidev/core';
    import Logo from './Logo.svelte';
</script>

<svelte:head>
    <title>Work Tracker</title>
    <meta name="description" content="Work Tracker web page" />
</svelte:head>

<AppShell>
    <Header height="1">
        <Flex justify={'space-between'}>
            <Group>
                <Logo />
                <Title order={1} size={'lg'}>Work Tracker</Title>
            </Group>
            <Group position={'right'} spacing={'xs'}>
                <Button variant={'subtle'}>Download</Button>
                <Button variant={'subtle'}>Blog</Button>
                <Button variant={'default'}>Github</Button>
            </Group>
        </Flex>
    </Header>

    <slot />

    <Footer height="50">
        <Center>© 2023 wagnrd</Center>
    </Footer>
</AppShell>

<style>
</style>
