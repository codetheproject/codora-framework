import Link from "next/link";

export default function Page() {
  return (
    <div className="text-center p-4 space-y-4">
      <h1>Codora docs</h1>
      <Link href="/docs">Docs</Link>
    </div>
  );
}
