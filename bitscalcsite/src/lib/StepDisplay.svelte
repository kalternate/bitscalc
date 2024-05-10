<script>
    import ValueDisplay from "./ValueDisplay.svelte";

    export let step;
</script>

<div>
    {#if step.left} 
    <div class="toptext pt-2 pl-7">
        <ValueDisplay value={step.left}/>
    </div>
    {/if}
    <div class="flex">
        <div class="w-6 text-right mr-1 font-mono">{step.op}</div>
        <ValueDisplay value={step.right}/>
    </div>
    <hr/>
    <div class="pl-7">
        <ValueDisplay value={step.result}/>
    </div>
</div>