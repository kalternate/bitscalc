<script>

    export let buttonChannel;


    export let pressed = false;
    export let value;

    if (pressed) {
        buttonChannel.set(value);
    }

    buttonChannel.subscribe(v => {pressed = v === value})

</script>

{#if pressed}
<button disabled={true} class="font-mono text-left center text-emerald-300" >
    <slot/>
</button>
{:else}
<button class="font-mono underline text-left text-zinc-400" on:click={buttonChannel.set(value)}>
    <slot/>
</button>
{/if}
