<script>
    export let token;
    export let format = "";
    export let highlight;
    export let canHighlight = true;

    let highlightNow = false;

    highlight.subscribe((tag) => {
        if (token.tag) {
            highlightNow = tag === token.tag
        }
    });

    let display = "";

    switch (format) {
        case "dec":
            display = token.format.dec;
            break;
        case "hex":
            display = token.format.hex;
            break;
        case "bin":
            display = token.format.bin;
            break;

        default:
            display = token.text
            break;
    }


    function mouseEnter() {
        if (token.tag) {
            highlight.set(token.tag);
        }
    }

    function mouseExit() {
        if (token.tag) {
            highlight.set(0);
        }
    }

</script>

{#if token.tag && canHighlight}
    <span on:mouseenter={mouseEnter} on:mouseleave={mouseExit} role="button" tabindex="0">
        {#if highlightNow}
            <span class="font-mono text-sky-300 underline decoration-zinc-600 hover:decoration-sky-300">{display}</span>
        {:else}
            <span class="font-mono hover:text-sky-300 underline decoration-zinc-600 hover:decoration-sky-300">{display}</span>
        {/if}
    </span>
{:else}
    {#if highlightNow}
            <span class="font-mono text-sky-300">{display}</span>
    {:else}
        <span class="font-mono">{display}</span>
    {/if}
{/if}
