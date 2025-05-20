import type { Metadata } from "next";
import { Geist, Geist_Mono } from "next/font/google";
import "./globals.css";

const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
});

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
});

export const metadata: Metadata = {
  title: "QMeet Control",
  description: "QMeet Application Control",
  icons: {
    icon: "/favicon.ico",
  },
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" className="bg-gradient-to-tr from-[#e0e7ff] via-white to-[#f0f4ff]">
      <body
        className={`${geistSans.variable} ${geistMono.variable} antialiased text-neutral-800`}
      >
        {children}
      </body>
    </html>
  );
}
