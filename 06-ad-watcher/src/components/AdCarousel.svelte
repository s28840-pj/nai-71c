<script lang="ts">
	import { fly } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { ChevronLeft, ChevronRight } from 'lucide-svelte';
	import ParodyAd from './ParodyAd.svelte';

	// Types
	interface Ad {
		id: number;
		title: string;
		tagline: string;
		cta: string;
		style: 'minimalist' | 'popup' | 'data' | 'corporate';
	}

	interface Props {
		isPaused?: boolean;
		adwatched?: () => void;
	}

	let { isPaused = false, adwatched = () => {} }: Props = $props();

	// Constants
	const PARODY_ADS: Ad[] = [
		{
			id: 1,
			title: 'CONSUME™',
			tagline: "You don't need it, but you'll buy it anyway. Because everyone else is.",
			cta: 'Submit to Desire',
			style: 'minimalist'
		},
		{
			id: 2,
			title: "🎉 YOU'RE THE 1,000,000th VISITOR! 🎉",
			tagline: 'Click here to claim your FREE* iPad! (*Terms apply. iPad not included.)',
			cta: 'CLICK NOW!!!',
			style: 'popup'
		},
		{
			id: 3,
			title: 'DataHarvest Pro™',
			tagline: 'We already know everything about you. This ad is just a formality.',
			cta: 'Accept All Cookies (and more)',
			style: 'data'
		},
		{
			id: 4,
			title: 'SynergyMax Solutions',
			tagline: 'Leveraging blockchain-enabled AI to disrupt the paradigm of human attention.',
			cta: 'Schedule a Synergy Call',
			style: 'corporate'
		},
		{
			id: 5,
			title: '⚠️ YOUR ATTENTION IS REQUIRED ⚠️',
			tagline: 'This mandatory advertisement cannot be skipped. Resistance is futile.',
			cta: 'Accept Your Fate',
			style: 'popup'
		},
		{
			id: 6,
			title: 'InfiniteScroll™ Premium',
			tagline: 'Why stop watching ads when you could watch more ads? Upgrade to see 2x more ads!',
			cta: 'More Ads Please',
			style: 'corporate'
		}
	];

	// State
	let currentIndex = $state(0);
	let direction = $state(0);

	// Navigation Logic
	const navigate = (newDirection: number) => {
		direction = newDirection;
		if (newDirection === 1) {
			currentIndex = (currentIndex + 1) % PARODY_ADS.length;
		} else {
			currentIndex = currentIndex === 0 ? PARODY_ADS.length - 1 : currentIndex - 1;
		}
		adwatched();
	};

	const goToIndex = (index: number) => {
		direction = index > currentIndex ? 1 : -1;
		currentIndex = index;
	};

	// Auto-advance Effect
	$effect(() => {
		if (isPaused) return;

		const interval = setInterval(() => {
			navigate(1);
		}, 5000);

		return () => clearInterval(interval);
	});
</script>

<div class="carousel-container">
	<!-- Ad container -->
	<!-- Grid layout used to stack slides on top of each other while preserving height -->
	<div class="track">
		{#key currentIndex}
			<div
				class="slide-wrapper"
				in:fly={{ x: direction * 300, duration: 300, easing: cubicOut, opacity: 0 }}
				out:fly={{ x: direction * -300, duration: 300, easing: cubicOut, opacity: 0 }}
			>
				<ParodyAd ad={PARODY_ADS[currentIndex]} {isPaused} />
			</div>
		{/key}
	</div>

	<!-- Navigation -->
	<div class="controls">
		<button
			class="nav-btn"
			onclick={() => navigate(-1)}
			disabled={isPaused}
			aria-label="Previous Ad"
		>
			<ChevronLeft size={20} />
		</button>

		<!-- Dots -->
		<div class="dots">
			{#each PARODY_ADS as _, index}
				<button
					class="dot"
					class:active={index === currentIndex}
					class:paused={isPaused}
					onclick={() => goToIndex(index)}
					disabled={isPaused}
					aria-label="Go to ad {index + 1}"
				></button>
			{/each}
		</div>

		<button class="nav-btn" onclick={() => navigate(1)} disabled={isPaused} aria-label="Next Ad">
			<ChevronRight size={20} />
		</button>
	</div>

	<!-- Progress bar for auto-advance -->
	{#if !isPaused}
		{#key currentIndex}
			<div class="progress-bar"></div>
		{/key}
	{/if}
</div>

<style>
	/* CSS Variables for easy theming */
	:global(:root) {
		--carousel-primary: #3b82f6; /* Blue-500 equivalent */
		--carousel-secondary: #e5e7eb; /* Gray-200 equivalent */
		--carousel-muted: #9ca3af; /* Gray-400 equivalent */
	}

	.carousel-container {
		position: relative;
		width: 100%;
	}

	/* Track Layout */
	.track {
		display: grid;
		min-height: 280px;
		overflow: hidden;
		position: relative;
	}

	.slide-wrapper {
		grid-column: 1;
		grid-row: 1;
		width: 100%;
	}

	/* Controls Layout */
	.controls {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-top: 1rem;
	}

	/* Buttons */
	.nav-btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: 0.5rem;
		border-radius: 0.25rem;
		background-color: var(--carousel-secondary);
		border: none;
		cursor: pointer;
		transition: background-color 0.2s;
		color: inherit;
	}

	.nav-btn:hover:not(:disabled) {
		background-color: color-mix(in srgb, var(--carousel-secondary), black 10%);
	}

	.nav-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* Dots */
	.dots {
		display: flex;
		gap: 0.5rem;
	}

	.dot {
		width: 0.5rem;
		height: 0.5rem;
		border-radius: 9999px;
		background-color: var(--carousel-muted);
		opacity: 0.3;
		border: none;
		padding: 0;
		cursor: pointer;
		transition:
			all 0.3s ease,
			opacity 0.2s;
	}

	.dot:hover:not(:disabled) {
		opacity: 0.5;
	}

	.dot.active {
		background-color: var(--carousel-primary);
		width: 1.5rem; /* Expands width when active */
		opacity: 1;
	}

	.dot.paused {
		opacity: 0.5;
	}

	/* Progress Bar */
	.progress-bar {
		position: absolute;
		bottom: 0;
		left: 0;
		height: 2px;
		background-color: var(--carousel-primary);
		animation: grow-width 5s linear forwards;
	}

	@keyframes grow-width {
		from {
			width: 0%;
		}
		to {
			width: 100%;
		}
	}
</style>
