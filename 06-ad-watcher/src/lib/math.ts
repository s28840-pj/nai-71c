// 1. Define a simple structure for a 3D Point/Vector
export interface Point3D {
	x: number;
	y: number;
	z: number;
}

// Subtract vector B from A (A - B)
function subtract(a: Point3D, b: Point3D): Point3D {
	return { x: a.x - b.x, y: a.y - b.y, z: a.z - b.z };
}

// Calculate the Cross Product (returns a vector perpendicular to both input vectors)
function cross(a: Point3D, b: Point3D): Point3D {
	return {
		x: a.y * b.z - a.z * b.y,
		y: a.z * b.x - a.x * b.z,
		z: a.x * b.y - a.y * b.x
	};
}

// Calculate the magnitude (length) of a vector
function magnitude(v: Point3D): number {
	return Math.sqrt(v.x * v.x + v.y * v.y + v.z * v.z);
}

// Normalize a vector (make its length 1)
function normalize(v: Point3D): Point3D {
	const mag = magnitude(v);
	if (mag === 0) return { x: 0, y: 0, z: 0 }; // Handle zero-length vectors
	return { x: v.x / mag, y: v.y / mag, z: v.z / mag };
}

// Calculate Dot Product (returns a scalar number)
function dot(a: Point3D, b: Point3D): number {
	return a.x * b.x + a.y * b.y + a.z * b.z;
}

/**
 * Calculates how much a plane (defined by 3 points) is facing the camera.
 *
 * @param p1 Point 1 on the plane
 * @param p2 Point 2 on the plane
 * @param p3 Point 3 on the plane
 * @param cameraPosition The location of the camera in 3D space
 * @returns A number between -1.0 and 1.0.
 *          1.0 = Plane is perfectly flat facing the camera.
 *          0.0 = Plane is perpendicular (you are looking at the edge).
 *          -1.0 = The plane is facing directly away from the camera.
 */
export function calculatePlaneFacingScore(
	p1: Point3D,
	p2: Point3D,
	p3: Point3D,
	cameraPosition: Point3D
): number {
	// Step 1: Calculate two vectors lying on the plane
	const v1 = subtract(p2, p1);
	const v2 = subtract(p3, p1);

	// Step 2: Calculate the Normal (The line perpendicular to the plane)
	// Note: The order of p1, p2, p3 matters (Winding Order).
	// Standard convention is Counter-Clockwise points produce a Normal facing "Out".
	const planeNormal = normalize(cross(v1, v2));

	// Step 3: Calculate the vector from the Plane to the Camera
	// We use p1 as the anchor point on the plane (any of the 3 would work)
	const vectorToCamera = subtract(cameraPosition, p1);
	const viewDirection = normalize(vectorToCamera);

	// Step 4: Calculate alignment using Dot Product
	// If Normal and ViewDirection are parallel (facing each other), dot is 1.
	const facingRatio = dot(planeNormal, viewDirection);

	return facingRatio;
}
