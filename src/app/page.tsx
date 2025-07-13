// app/page.tsx - Landing Page
import Link from 'next/link';
import { ArrowRight, Database, BarChart3, Trophy, Users } from 'lucide-react';

export default function LandingPage() {
  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-900 via-blue-900 to-slate-900">
      {/* Header */}
      <header className="relative z-10">
        <nav className="mx-auto max-w-7xl px-6 lg:px-8">
          <div className="flex h-16 items-center justify-between">
            <div className="flex items-center space-x-3">
              <div className="w-8 h-8 bg-blue-500 rounded-lg flex items-center justify-center">
                <span className="text-white text-lg">🏒</span>
              </div>
              <span className="text-white font-bold text-xl">Hockey DB</span>
            </div>
            <Link
              href="/auth/signin"
              className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg font-medium transition-colors"
            >
              Login
            </Link>
          </div>
        </nav>
      </header>

      {/* Hero Section */}
      <main>
        <div className="relative isolate px-6 pt-14 lg:px-8">
          <div className="mx-auto max-w-4xl py-24 sm:py-32">
            <div className="text-center">
              <h1 className="text-4xl font-bold tracking-tight text-white sm:text-6xl">
                International Hockey Database
              </h1>
              <p className="mt-6 text-lg leading-8 text-gray-300">
                Track hockey tournaments from around the world. Comprehensive database for events, 
                seasons, team participations, and player contracts across countries and competitions. 
                From local leagues to international championships.
              </p>
              <div className="mt-10 flex items-center justify-center gap-x-6">
                <Link
                  href="/auth/signin"
                  className="bg-blue-600 px-6 py-3 text-lg font-semibold text-white shadow-sm hover:bg-blue-500 rounded-lg transition-colors flex items-center space-x-2"
                >
                  <span>Get Started</span>
                  <ArrowRight className="h-5 w-5" />
                </Link>
              </div>
            </div>
          </div>

          {/* Features Grid */}
          <div className="mx-auto max-w-7xl px-6 lg:px-8 pb-24">
            <div className="mx-auto max-w-2xl text-center">
              <h2 className="text-3xl font-bold tracking-tight text-white sm:text-4xl">
                Everything you need to track international hockey
              </h2>
              <p className="mt-6 text-lg leading-8 text-gray-300">
                Professional database designed for hockey enthusiasts and historians
              </p>
            </div>
            <div className="mx-auto mt-16 max-w-2xl sm:mt-20 lg:mt-24 lg:max-w-none">
              <dl className="grid max-w-xl grid-cols-1 gap-x-8 gap-y-16 lg:max-w-none lg:grid-cols-4">
                <div className="flex flex-col">
                  <dt className="flex items-center gap-x-3 text-base font-semibold leading-7 text-white">
                    <Users className="h-5 w-5 flex-none text-blue-400" />
                    International Coverage
                  </dt>
                  <dd className="mt-4 flex flex-auto flex-col text-base leading-7 text-gray-300">
                    <p className="flex-auto">
                      Track teams and players across multiple countries. Support for national teams, 
                      club teams, and special all-star selections.
                    </p>
                  </dd>
                </div>
                <div className="flex flex-col">
                  <dt className="flex items-center gap-x-3 text-base font-semibold leading-7 text-white">
                    <Database className="h-5 w-5 flex-none text-blue-400" />
                    Event & Season Management
                  </dt>
                  <dd className="mt-4 flex flex-auto flex-col text-base leading-7 text-gray-300">
                    <p className="flex-auto">
                      Organize events by country or international scope. Track seasons with 
                      sponsor names and team participations across years.
                    </p>
                  </dd>
                </div>
                <div className="flex flex-col">
                  <dt className="flex items-center gap-x-3 text-base font-semibold leading-7 text-white">
                    <BarChart3 className="h-5 w-5 flex-none text-blue-400" />
                    Player Contract History
                  </dt>
                  <dd className="mt-4 flex flex-auto flex-col text-base leading-7 text-gray-300">
                    <p className="flex-auto">
                      Complete player career tracking with contract history across different 
                      teams, seasons, and competitions.
                    </p>
                  </dd>
                </div>
                <div className="flex flex-col">
                  <dt className="flex items-center gap-x-3 text-base font-semibold leading-7 text-white">
                    <Trophy className="h-5 w-5 flex-none text-blue-400" />
                    Tournament Archives
                  </dt>
                  <dd className="mt-4 flex flex-auto flex-col text-base leading-7 text-gray-300">
                    <p className="flex-auto">
                      Comprehensive historical database for championships, international 
                      tournaments, and multi-country competitions.
                    </p>
                  </dd>
                </div>
              </dl>
            </div>
          </div>
        </div>
      </main>

      {/* Footer */}
      <footer className="relative">
        <div className="mx-auto max-w-7xl px-6 py-12 lg:px-8">
          <div className="border-t border-gray-700 pt-8">
            <p className="text-center text-sm text-gray-400">
              International Hockey Database - Built for hockey enthusiasts and historians
            </p>
          </div>
        </div>
      </footer>
    </div>
  );
}