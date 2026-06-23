<script lang="ts">
	import { networkStream } from "$lib/app_builder";
	import "../app.css";

	let { children } = $props();
	networkStream.run();
</script>

{@render children()}
