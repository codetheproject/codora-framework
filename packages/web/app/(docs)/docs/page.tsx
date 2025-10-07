import { Link } from "nextra-theme-docs";

export default function () {
	return (
		<div className="p-10 min-h-screen">
			<p>We got this working</p>
			<Link href={"/docs/getting-started"}>Getting Started</Link>
		</div>
	);
}
