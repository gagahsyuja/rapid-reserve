<script lang="ts">
    import Fa from 'svelte-fa';
    import { faXmark } from '@fortawesome/free-solid-svg-icons';
</script>

<button on:click class="absolute top-0 right-0 m-8">
    <Fa icon={faXmark} class="text-everforest-green text-2xl hover:text-everforest-red hover:duration-200" />
</button>
