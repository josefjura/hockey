// app/[locale]/page.tsx - Localized Landing Page
import Link from 'next/link';
import { ArrowRight, Database, BarChart3, Trophy, Users } from 'lucide-react';
import { useTranslations } from 'next-intl';

export default function LandingPage() {
  const t = useTranslations('LandingPage');
  
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
              <span className="text-white font-bold text-xl">{t('title')}</span>
            </div>
            <Link
              href="/auth/signin"
              className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg font-medium transition-colors"
            >
              {t('login')}
            </Link>
          </div>
        </nav>
      </header>

      {/* Hero Section */}
      <main>
        <div className="relative isolate px-6 pt-14 lg:px-8">
          <div className="mx-auto max-w-2xl py-32 sm:py-48 lg:py-56">
            <div className="text-center">
              <h1 className="text-4xl font-bold tracking-tight text-white sm:text-6xl">
                {t('hero.title')}
              </h1>
              <p className="mt-6 text-lg leading-8 text-gray-300">
                {t('hero.subtitle')}
              </p>
              <div className="mt-10 flex items-center justify-center gap-x-6">
                <Link
                  href="/auth/signin"
                  className="bg-blue-600 px-6 py-3 text-sm font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 rounded-lg flex items-center space-x-2"
                >
                  <span>{t('hero.getStarted')}</span>
                  <ArrowRight className="w-4 h-4" />
                </Link>
              </div>
            </div>
          </div>
        </div>

        {/* Features Section */}
        <div className="py-24 sm:py-32">
          <div className="mx-auto max-w-7xl px-6 lg:px-8">
            <div className="mx-auto max-w-2xl text-center">
              <h2 className="text-3xl font-bold tracking-tight text-white sm:text-4xl">
                {t('features.title')}
              </h2>
              <p className="mt-6 text-lg leading-8 text-gray-300">
                {t('features.subtitle')}
              </p>
            </div>
            <div className="mx-auto mt-16 max-w-2xl sm:mt-20 lg:mt-24 lg:max-w-none">
              <dl className="grid max-w-xl grid-cols-1 gap-x-8 gap-y-16 lg:max-w-none lg:grid-cols-4">
                <div className="flex flex-col items-center text-center">
                  <div className="mb-6 flex h-16 w-16 items-center justify-center rounded-lg bg-blue-600">
                    <Database className="h-8 w-8 text-white" />
                  </div>
                  <dt className="text-base font-semibold leading-7 text-white">
                    {t('features.data.title')}
                  </dt>
                  <dd className="mt-4 flex flex-auto flex-col text-base leading-7 text-gray-300">
                    <p className="flex-auto">{t('features.data.description')}</p>
                  </dd>
                </div>
                <div className="flex flex-col items-center text-center">
                  <div className="mb-6 flex h-16 w-16 items-center justify-center rounded-lg bg-blue-600">
                    <BarChart3 className="h-8 w-8 text-white" />
                  </div>
                  <dt className="text-base font-semibold leading-7 text-white">
                    {t('features.analytics.title')}
                  </dt>
                  <dd className="mt-4 flex flex-auto flex-col text-base leading-7 text-gray-300">
                    <p className="flex-auto">{t('features.analytics.description')}</p>
                  </dd>
                </div>
                <div className="flex flex-col items-center text-center">
                  <div className="mb-6 flex h-16 w-16 items-center justify-center rounded-lg bg-blue-600">
                    <Trophy className="h-8 w-8 text-white" />
                  </div>
                  <dt className="text-base font-semibold leading-7 text-white">
                    {t('features.leagues.title')}
                  </dt>
                  <dd className="mt-4 flex flex-auto flex-col text-base leading-7 text-gray-300">
                    <p className="flex-auto">{t('features.leagues.description')}</p>
                  </dd>
                </div>
                <div className="flex flex-col items-center text-center">
                  <div className="mb-6 flex h-16 w-16 items-center justify-center rounded-lg bg-blue-600">
                    <Users className="h-8 w-8 text-white" />
                  </div>
                  <dt className="text-base font-semibold leading-7 text-white">
                    {t('features.teams.title')}
                  </dt>
                  <dd className="mt-4 flex flex-auto flex-col text-base leading-7 text-gray-300">
                    <p className="flex-auto">{t('features.teams.description')}</p>
                  </dd>
                </div>
              </dl>
            </div>
          </div>
        </div>
      </main>
    </div>
  );
}
