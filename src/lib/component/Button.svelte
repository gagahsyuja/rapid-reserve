<script lang="ts">
    export let name: string = "yes";
    export let fg: string = "everforest-black";
    export let bg: string = "everforest-green";
</script>

<button on:click class="bg-{bg} p-4 text-{fg} mx-[120px] rounded-md">
    {name}
</button>
