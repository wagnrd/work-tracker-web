<script lang="ts">
    import classNames from 'classnames';
    import './adwaita.css';

    export let type:
        | 'title-1'
        | 'title-2'
        | 'title-3'
        | 'title-4'
        | 'title-5'
        | 'heading'
        | 'body'
        | 'caption-heading'
        | 'caption' = 'body';
</script>

<span class={classNames(type)}>
    <slot />
</span>

<style>
    span.title-1 {
        font: var(--adw-font-title-1);
    }

    span.title-2 {
        font: var(--adw-font-title-2);
    }

    span.title-3 {
        font: var(--adw-font-title-3);
    }

    span.title-4 {
        font: var(--adw-font-title-4);
    }

    span.heading {
        font: var(--adw-font-heading);
    }

    span.body {
        font: var(--adw-font-body);
    }

    span.caption-heading {
        font: var(--adw-font-caption-heading);
    }

    span.caption {
        font: var(--adw-font-caption);
    }
</style>
