<script lang="ts">
    import classNames from 'classnames';
    import './adwaita.css';
    import Label from './Label.svelte';

    export let type: 'regular' | 'flat' | 'suggested-action' | 'destructive-action' = 'regular';
    export let shape: 'rectangle' | 'pill' | 'circular' = 'rectangle';
</script>

<button class={classNames(type, shape)}>
    <Label type="heading">
        <slot />
    </Label>
</button>

<style>
    button {
        border: none;
        min-height: 24px;
        min-width: 16px;
    }

    button.rectangle {
        padding: 7px 10px 9px;
        border-radius: 6px;
    }

    button.pill {
        padding: 12px 32px 14px;
        border-radius: 99px; /* just some high number to always be completely round */
    }

    button.circular {
        width: 35px;
        height: 35px;
        padding: 7px 10px 9px;
        border-radius: 100%;
    }

    button.regular {
        background: var(--adw-color-element-bg);
    }

    button.flat {
        background: none;
    }

    button.suggested-action {
        background: var(--adw-color-accent-bg);
        color: var(--adw-color-accent-fg);
    }

    button.destructive-action {
        background: var(--adw-color-destructive-bg);
        color: var(--adw-color-destructive-fg);
    }
</style>
