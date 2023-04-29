<script>
	import '../app.css';
	import { init } from '$lib/ws';
	import { onMount } from 'svelte';

	onMount(init);
</script>

<slot />
