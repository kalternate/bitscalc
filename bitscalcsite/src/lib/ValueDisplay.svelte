<script>
    export let value;
    export let format="dec";
    
    let formattedValue = "";

    if (format === "dec") {
        formattedValue = value.dec;
    } else if (format === "hex") {
        formattedValue = value.hex;
    } else if (format === "bin") {
        formattedValue = value.hex;
    }
</script>

<div class="font-mono">{formattedValue}</div>

