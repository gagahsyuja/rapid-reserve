<script lang="ts">
    export let message: string = 'Are you sure?';
</script>

<div class="absolute flex flex-col items-center justify-center z-10 left-0 top-0 w-full h-full backdrop-blur-3xl bg-everforest-black/90">
    <div class="relative w-[20%] h-auto bg-everforest-black border-4 p-4 py-8 border-everforest-green rounded-xl shadow-lg shadow-black">
        <h1 class="text-3xl font-semibold text-center mb-6">Are you sure?</h1>
        <h1 class="text-xl font-medium text-center mb-6">{message}</h1>
        <slot />
    </div>
</div>
