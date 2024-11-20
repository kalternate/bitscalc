<script>
    export let format;
    export let myFormat;

    function onClick() {
        format = myFormat;
    }

</script>

{#if format === myFormat}
    <button on:click={onClick} class="font-mono text-sm">
        <slot/>
    </button>
{:else}
    <button on:click={onClick} class="font-mono underline text-sm text-zinc-400">
        <slot/>
    </button>
{/if}
