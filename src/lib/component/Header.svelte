<script lang="ts">
    export let showAddButton: boolean = false;
    export let searchKeyword: string = '';
    export let whatToAdd: string = 'Somethin';
</script>

<div class="flex flex-row align-center items-center justify-between px-8">
    <button
        on:click={() => showAddButton = true}
        class="py-2 px-4 bg-everforest-blue shadow-md shadow-black text-everforest-black rounded-md h-[40px] text-md"
    >
        Add {whatToAdd}
    </button>
    <input
        class="bg-everforest-blue shadow-md shadow-black focus:outline-none rounded-l-2xl rounded-r-md h-[40px] text-everforest-black placeholder:text-everforest-black px-6 py-1"
        type="text"
        placeholder="Search..."
        bind:value={searchKeyword}
    />
</div>
