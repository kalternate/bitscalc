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
    <button on:click={onClick} class="font-mono hover:text-sky-300 underline text-sm text-zinc-400">
        <slot/>
    </button>
{/if}
