<script>
    import TokenDisplay from "./TokenDisplay.svelte";

    export let step;
    export let highlight;

</script>

<div>
    {#if step.left} 
    <div class="toptext pt-2 pl-7 text-right">
        <TokenDisplay token={step.left} highlight={highlight}/>
    </div>
    {/if}
    <div class="flex">
        <div class="w-6 mr-1 font-mono text-right">{step.op}</div>
        <div class="text-right flex-grow">
            <TokenDisplay token={step.right} highlight={highlight}/>
        </div>
    </div>
    <hr/>
    <div class="pl-7 text-right">
        <TokenDisplay token={step.result} highlight={highlight}/>    
    </div>
</div>