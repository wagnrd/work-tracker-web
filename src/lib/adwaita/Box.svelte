<script lang="ts">
    import './adwaita.css';
    import classNames from 'classnames';
    import type { Orientation } from './types';

    export let orientation: Orientation = 'horizontal';
    export let linked = false;
    export let spacing = 0;
</script>

<div class={classNames(orientation, { linked })} style="gap: {spacing}">
    <slot />
</div>

<style>
    div {
        position: relative;
        display: flex;
        width: fit-content;
        height: fit-content;
    }

    div.vertical {
        flex-direction: column;
    }

    div.linked :global(> :not(:last-child, :first-child)) {
        border-radius: 0 !important;
    }

    div.linked.horizontal :global(> :first-child) {
        border-top-right-radius: 0 !important;
        border-bottom-right-radius: 0 !important;
    }

    div.linked.horizontal :global(> :last-child) {
        border-top-left-radius: 0 !important;
        border-bottom-left-radius: 0 !important;
    }

    div.linked.horizontal :global(> :not(:last-child)) {
        border-right: 1px solid var(--adw-color-border);
    }

    div.linked.vertical :global(> :first-child) {
        border-bottom-left-radius: 0 !important;
        border-bottom-right-radius: 0 !important;
    }

    div.linked.vertical :global(> :last-child) {
        border-top-left-radius: 0 !important;
        border-top-right-radius: 0 !important;
    }

    div.linked.vertical :global(> :not(:last-child)) {
        border-bottom: 1px solid var(--adw-color-border);
    }
</style>
