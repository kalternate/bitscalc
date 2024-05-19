<script>
    import TokenDisplay from "./TokenDisplay.svelte";

    export let step;
    export let highlight;
    export let format = "";
</script>

<div class="">
    {#if step.left} 
    <div class="toptext pt-2 pl-7 text-right">
        <TokenDisplay token={step.left} highlight={highlight} format={format}/>
    </div>
    {/if}
    <div class="flex">
        <div class="w-6 mr-1 font-mono text-right">{step.op}</div>
        <div class="text-right flex-grow">
            <TokenDisplay token={step.right} highlight={highlight} format={format}/>
        </div>
    </div>
    <hr/>
    <div class="pl-7 text-right">
        <TokenDisplay token={step.result} highlight={highlight} format={format}/>    
    </div>
</div>