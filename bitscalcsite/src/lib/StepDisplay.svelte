<script>
    import TokenDisplay from "./TokenDisplay.svelte";

    export let step;
    export let highlight;
    export let commandChannel;

</script>

<div>
    {#if step.left} 
    <div class="toptext pt-2 pl-7 text-right">
        <TokenDisplay token={step.left} highlight={highlight} commandChannel={commandChannel}/>
    </div>
    {/if}
    <div class="flex">
        <div class="w-6 mr-1 font-mono text-right">{step.op}</div>
        <div class="text-right flex-grow">
            <TokenDisplay token={step.right} highlight={highlight} commandChannel={commandChannel}/>
        </div>
    </div>
    <hr/>
    <div class="pl-7 text-right">
        <TokenDisplay token={step.result} highlight={highlight} commandChannel={commandChannel}/>    
    </div>
</div>