<script lang="ts">
	import { fade } from 'svelte/transition';

	// Types
	interface AdProps {
		ad: {
			id: number;
			title: string;
			tagline: string;
			cta: string;
			style: 'corporate' | 'popup' | 'minimalist' | 'data';
		};
		isPaused: boolean;
	}

	let { ad, isPaused }: AdProps = $props();
</script>

<!-- 
  data-style allows us to scope CSS specific to the ad type 
  class:paused handles the conditional state styling
-->
<div class="ad-card" data-style={ad.style} class:paused={isPaused}>
	<!-- Paused overlay -->
	{#if isPaused}
		<div class="overlay" transition:fade={{ duration: 200 }}>
			<div class="overlay-content">
				<div class="pause-title">⏸ AD PAUSED</div>
				<p class="pause-subtitle">ATTENTION REQUIRED</p>
			</div>
		</div>
	{/if}

	<div class="content">
		<h3>
			{ad.title}
		</h3>

		<p class="tagline">
			{ad.tagline}
		</p>

		<button class="cta-button" disabled={isPaused}>
			{ad.cta}
		</button>
	</div>

	<!-- Decorative elements based on style -->
	{#if ad.style === 'popup'}
		<div class="popup-close">✕ (doesn't work)</div>
		<div class="popup-id">AD #00{ad.id}</div>
	{/if}

	{#if ad.style === 'data'}
		<div class="data-badge">
			<span class="pulse-dot"></span>
			COLLECTING
		</div>
	{/if}
</div>

<style>
	/* --- Theming Variables --- */
	/* These mimic the Tailwind utility colors from the original */
	.ad-card {
		--c-bg: #ffffff;
		--c-text: #0f172a; /* slate-900 */
		--c-muted: #64748b; /* slate-500 */
		--c-border: #e2e8f0; /* slate-200 */
		--c-primary: #0f172a;
		--c-primary-fg: #ffffff;
		--c-warning: #f59e0b; /* amber-500 */
		--c-warning-fg: #ffffff;
		--c-surveillance: #10b981; /* emerald-500 */
		--font-mono: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;

		/* Base Layout */
		position: relative;
		padding: 1.5rem;
		border-radius: 0.5rem;
		overflow: hidden;
		height: 100%;
		display: flex;
		flex-direction: column;
		justify-content: center;

		/* Transitions for state changes (replaces framer-motion animate) */
		transition:
			opacity 0.3s ease,
			transform 0.3s ease,
			filter 0.3s ease;
		transform-origin: center;
		border: 1px solid var(--c-border);
		background-color: var(--c-bg);
	}

	/* --- Paused State --- */
	.ad-card.paused {
		opacity: 0.3;
		transform: scale(0.98);
		filter: grayscale(100%);
		animation: pulse-fast 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
	}

	/* --- Style Variants --- */

	/* 1. Corporate Style */
	.ad-card[data-style='corporate'] {
		background: linear-gradient(to bottom right, #f1f5f9, #cbd5e1); /* secondary to muted */
		border-width: 2px;
	}

	/* 2. Popup Style */
	.ad-card[data-style='popup'] {
		background-color: var(--c-warning);
		color: var(--c-warning-fg);
		border: 4px solid var(--c-primary);
	}
	.ad-card[data-style='popup'] h3,
	.ad-card[data-style='popup'] .tagline {
		color: var(--c-warning-fg);
	}
	.ad-card[data-style='popup'] .cta-button {
		background-color: var(--c-primary);
		color: var(--c-primary-fg);
	}

	/* 3. Minimalist Style */
	.ad-card[data-style='minimalist'] {
		background-color: var(--c-bg);
		border: 1px solid var(--c-border);
	}

	/* 4. Data Style */
	.ad-card[data-style='data'] {
		background: linear-gradient(
			to bottom right,
			#e2e8f0,
			#ffffff,
			#f1f5f9
		); /* muted via card to secondary */
		border-color: rgba(16, 185, 129, 0.3); /* surveillance/30 */
	}

	/* --- Content Typography --- */
	.content {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		position: relative;
		z-index: 1; /* Ensure content is above background */
	}

	h3 {
		font-size: 1.5rem;
		line-height: 2rem;
		font-weight: 900;
		letter-spacing: -0.025em;
		margin: 0;
		color: var(--c-text);
	}

	.tagline {
		font-size: 1.125rem;
		line-height: 1.75rem;
		color: var(--c-muted);
		margin: 0;
	}

	/* --- Button --- */
	.cta-button {
		align-self: flex-start;
		padding: 0.75rem 1.5rem;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		border-radius: 0.25rem;
		background-color: var(--c-primary);
		color: var(--c-primary-fg);
		border: none;
		cursor: pointer;
		transition:
			transform 0.1s ease,
			background-color 0.2s ease;
	}

	.cta-button:hover:not(:disabled) {
		transform: scale(1.05);
		opacity: 0.9;
	}

	.cta-button:active:not(:disabled) {
		transform: scale(0.95);
	}

	.cta-button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* --- Overlay --- */
	.overlay {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		background-color: rgba(255, 255, 255, 0.8);
		z-index: 10;
		border-radius: 0.25rem;
	}

	.overlay-content {
		text-align: center;
	}

	.pause-title {
		font-size: 1.5rem;
		font-weight: 700;
		color: #ef4444; /* Red */
		text-shadow: 0 0 10px rgba(239, 68, 68, 0.5);
		animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
	}

	.pause-subtitle {
		font-size: 0.875rem;
		color: var(--c-muted);
		margin-top: 0.25rem;
		font-family: var(--font-mono);
		font-weight: 600;
	}

	/* --- Decorative Elements --- */
	.popup-close {
		position: absolute;
		top: 0.5rem;
		right: 0.5rem;
		font-size: 0.75rem;
		font-weight: 700;
		color: var(--c-warning-fg);
		opacity: 0.6;
	}

	.popup-id {
		position: absolute;
		bottom: 0.5rem;
		left: 0.5rem;
		font-size: 0.75rem;
		font-family: var(--font-mono);
		color: var(--c-warning-fg);
		opacity: 0.7;
	}

	.data-badge {
		position: absolute;
		top: 0.5rem;
		right: 0.5rem;
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-size: 0.75rem;
		color: var(--c-surveillance);
		font-family: var(--font-mono);
	}

	.pulse-dot {
		width: 0.5rem;
		height: 0.5rem;
		border-radius: 9999px;
		background-color: var(--c-surveillance);
		animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
	}

	/* --- Animations --- */
	@keyframes pulse {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.5;
		}
	}

	@keyframes pulse-fast {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.8; /* Subtle pulse for the whole card */
		}
	}
</style>
