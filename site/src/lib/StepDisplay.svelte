<script>
    import TokenDisplay from "./TokenDisplay.svelte";

    export let step;
    export let highlight;
    export let commandChannel;
    export let format = "";
</script>

<div class="">
    {#if step.left} 
    <div class="toptext pt-2 pl-7 text-right">
        <TokenDisplay token={step.left} highlight={highlight} commandChannel={commandChannel} format={format}/>
    </div>
    {/if}
    <div class="flex">
        <div class="w-6 mr-1 font-mono text-right">{step.op}</div>
        <div class="text-right flex-grow">
            <TokenDisplay token={step.right} highlight={highlight} commandChannel={commandChannel} format={format}/>
        </div>
    </div>
    <hr/>
    <div class="pl-7 text-right">
        <TokenDisplay token={step.result} highlight={highlight} commandChannel={commandChannel} format={format}/>    
    </div>
</div>