<script>
    import ValueDisplay from "./ValueDisplay.svelte";

    export let step;
</script>

<div>
    {#if step.left} 
    <div class="toptext pt-2 pl-7 text-right">
        <ValueDisplay value={step.left}/>
    </div>
    {/if}
    <div class="flex">
        <div class="w-6 mr-1 font-mono text-right">{step.op}</div>
        <div class="text-right flex-grow">
            <ValueDisplay value={step.right}/>
        </div>
    </div>
    <hr/>
    <div class="pl-7 text-right">
        <ValueDisplay value={step.result}/>
    </div>
</div>