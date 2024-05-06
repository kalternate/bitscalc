<script>
    export let value;
</script>

<div class="flex flex-col">
    <div class="flex flex-row space-x-2 items-center">
        <div class="w-6 text-right montserrat text-xs text-zinc-400 align-middle">DEC</div>
        <div class="font-mono">{value.dec}</div>
    </div>
    <div class="flex flex-row space-x-2 items-center">
        <div class="w-6 text-right montserrat text-xs text-zinc-400 align-middle">HEX</div>
        <div class="font-mono">{value.hex}</div>
    </div>
    <div class="flex flex-row space-x-2 items-center">
        <div class="w-6 text-right montserrat text-xs text-zinc-400 align-middle">BIN</div>
        <div class="font-mono">{value.bin}</div>
    </div>
</div>

