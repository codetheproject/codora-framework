import type { Metadata } from "next";
import { Poppins } from "next/font/google";
import "../globals.css";
import type { ReactNode } from "react";

const font = Poppins({
	variable: "--font-sans",
	subsets: ["latin"],
	weight: ["100", "200", "300", "400", "500", "600", "700", "800", "900"],
});

export const metadata: Metadata = {
	title: "Codora Framework",
	description: "Open source software to build Rust application faster",
};

export default function ({ children }: Readonly<{ children: ReactNode }>) {
	return (
		<html lang="en">
			<body className={`${font.variable} antialiased`}>{children}</body>
		</html>
	);
}
