<script>
    import Logo from '$lib/Logo.svelte';
    import Box from '$lib/adwaita/Box.svelte';
    import Button from '$lib/adwaita/Button.svelte';
    import Label from '$lib/adwaita/Label.svelte';
</script>

<svelte:head>
    <title>Work Tracker</title>
    <meta name="description" content="Work Tracker web page" />
</svelte:head>

<Logo />
<div>
    <Button>Open</Button>
    <Button type="flat">Flat</Button>
    <Button type="suggested-action">Suggested</Button>
    <Button type="destructive-action">Destructive</Button>
    <Button shape="pill">Pill Button</Button>
    <Button shape="circular">A</Button>
</div>
<div>
    <Label type="title-1">Title 1</Label>
    <Label type="title-2">Title 2</Label>
    <Label type="title-3">Title 3</Label>
    <Label type="title-4">Title 4</Label>
    <Label type="heading">Heading</Label>
    <Label type="body">Body</Label>
    <Label type="caption-heading">Caption Heading</Label>
    <Label type="caption">Caption</Label>
</div>
<Box linked orientation="horizontal">
    <Button>1</Button>
    <Button>2</Button>
    <Button>3</Button>
</Box>
<Box linked orientation="vertical">
    <Button>1</Button>
    <Button>2</Button>
    <Button>3</Button>
</Box>

<main>
    <slot />
</main>

<style>
    :global(html) {
        margin: 0;
    }

    main {
        margin: 50px 0;
    }
</style>
