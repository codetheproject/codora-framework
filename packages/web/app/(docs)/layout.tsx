import { Banner, Head } from "nextra/components";
import { getPageMap } from "nextra/page-map";
import { Footer, Layout, Navbar } from "nextra-theme-docs";
import "nextra-theme-docs/style.css";
import "../globals.css";
import type { Metadata } from "next";
import type { ReactNode } from "react";

export const metadata: Metadata = {
	title: "Valar Framework Docs",
	description: "Open source software to build Rust application faster",
};

export default async function ({ children }: { children: ReactNode }) {
	return (
		<html
			// Not required, but good for SEO
			lang="en"
			// Required to be set
			dir="ltr"
			// Suggested by `next-themes` package https://github.com/pacocoursey/next-themes#with-app
			suppressHydrationWarning
		>
			<Head
			// ... Your additional head options
			>
				{/* Your additional tags should be passed as `children` of `<Head>` element */}
			</Head>
			<body>
				<Layout
					banner={<Banner>Valar Framework</Banner>}
					navbar={
						<Navbar
							logo={<b>Valar Framework</b>}
							// ... Your additional navbar options
							chatIcon="Discord"
							chatLink="https://discord.com/valar"
							projectIcon="Github"
							projectLink="https://github.com/codetheproject/valar"
						/>
					}
					pageMap={await getPageMap()}
					docsRepositoryBase="https://github.com/codetheproject/valar/packages/web"
					footer={<Footer>MIT {new Date().getFullYear()} © Valar.</Footer>}
					// ... Your additional layout options
				>
					{children}
				</Layout>
			</body>
		</html>
	);
}
