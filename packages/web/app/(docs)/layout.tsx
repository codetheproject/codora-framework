import { Banner, Head } from "nextra/components";
import { getPageMap } from "nextra/page-map";
import { Footer, Layout, Navbar } from "nextra-theme-docs";
import "nextra-theme-docs/style.css";
import "../globals.css";
import type { Metadata } from "next";
import type { ReactNode } from "react";

export const metadata: Metadata = {
	title: "Codora Framework Docs",
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
					banner={<Banner>Codora Framework</Banner>}
					navbar={
						<Navbar
							logo={<b>Codora Framework</b>}
							// ... Your additional navbar options
							chatIcon="Discord"
							chatLink="https://discord.com/codora-framework"
							projectIcon="Github"
							projectLink="https://github.com/codetheproject/codora-framework"
						/>
					}
					pageMap={await getPageMap()}
					docsRepositoryBase="https://github.com/codetheproject/codora-framework/packages/web"
					footer={<Footer>MIT {new Date().getFullYear()} © Codora.</Footer>}
					// ... Your additional layout options
				>
					{children}
				</Layout>
			</body>
		</html>
	);
}
