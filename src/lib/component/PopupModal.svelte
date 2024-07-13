<script lang="ts">
    export let title: any = null;
</script>

<div class="z-10 absolute flex flex-col items-center justify-center z-10 left-0 top-0 w-full h-full backdrop-blur-3xl bg-everforest-black/90">
    <div class="relative w-[50%] h-auto bg-everforest-black border-4 border-everforest-green rounded-xl px-[10%] py-[2%] shadow-lg shadow-black">
            {#if title}
                <h1 class="text-4xl font-bold text-center p-0">{title}</h1>
            {/if}
        <slot />
    </div>
</div>
