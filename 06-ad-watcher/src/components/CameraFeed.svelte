<script lang="ts">
	import * as fld from '@tensorflow-models/face-landmarks-detection';
	import * as faceMesh from '@mediapipe/face_mesh';
	import * as tfjsWasm from '@tensorflow/tfjs-backend-wasm';
	import '@tensorflow-models/face-detection';
	import '@tensorflow/tfjs-backend-webgl';
	import { onMount } from 'svelte';
	import { calculatePlaneFacingScore, type Point3D } from '$lib/math';

	interface Props {
		statechanged?: (faceDetected: boolean) => void;
	}

	let { statechanged = () => {} }: Props = $props();

	let isFocused = $state(!document.hidden);
	let faceDetected = $state(false);

	async function getCameraStream(): Promise<MediaStream> {
		return navigator.mediaDevices.getUserMedia({
			audio: false,
			video: {
				facingMode: 'user',
				width: 360,
				height: 270
			}
		});
	}

	let video: HTMLVideoElement;
	const model = fld.SupportedModels.MediaPipeFaceMesh;
	let detector: fld.FaceLandmarksDetector;
	let cameraStream = $state<MediaStream | undefined>(undefined);
	onMount(async () => {
		tfjsWasm.setWasmPaths(
			`https://cdn.jsdelivr.net/npm/@tensorflow/tfjs-backend-wasm@${tfjsWasm.version_wasm}/dist/`
		);

		detector = await fld.createDetector(model, {
			runtime: 'mediapipe',
			maxFaces: 1,
			refineLandmarks: false,
			solutionPath: `https://cdn.jsdelivr.net/npm/@mediapipe/face_mesh@${faceMesh.VERSION}`
		});
		cameraStream = await getCameraStream();
	});

	const CAMERA_POS = {
		x: 360 / 2,
		y: 270 / 2,
		z: -10
	};

	function startDetection(initialTimestamp: number) {
		faceDetectionLoop(initialTimestamp);
	}

	async function faceDetectionLoop(timestamp: number) {
		if (isFocused) {
			let nextState = false;
			try {
				const faces = await detector.estimateFaces(video, { flipHorizontal: false });

				if (faces.length > 0) {
					const face = faces[0];
					const leftEyePoints = face.keypoints.filter((k) => k.name === 'leftEye') as Point3D[];
					const rightEyePoints = face.keypoints.filter((k) => k.name === 'rightEye') as Point3D[];

					if (leftEyePoints.length !== 0 && rightEyePoints.length !== 0) {
						const a = leftEyePoints[0];
						const b = leftEyePoints[1];
						const c = rightEyePoints[0];
						const score = calculatePlaneFacingScore(a, b, c, CAMERA_POS);
						nextState = score > 0.1;
					}
				}

				if (faceDetected != nextState) {
					faceDetected = nextState;
					statechanged(nextState);
				}
			} catch (e) {}

			requestAnimationFrame(faceDetectionLoop);
		}
	}
</script>

<svelte:document
	onvisibilitychange={() => {
		if (document.hidden) {
			statechanged(false);
			isFocused = false;
		} else {
			statechanged(faceDetected);
			isFocused = true;
		}
	}}
/>

<video
	bind:this={video}
	srcobject={cameraStream}
	playsinline
	autoplay
	onloadeddata={() => requestAnimationFrame(startDetection)}
></video>

<style>
	video {
		width: 100%;
		height: auto;
	}
</style>
