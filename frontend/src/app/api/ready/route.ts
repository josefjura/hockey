import { NextResponse } from 'next/server'

export async function GET() {
  try {
    const backendUrl = process.env.HOCKEY_BACKEND_URL || 'http://localhost:8080'
    
    // Check if we can reach the backend health endpoint
    const response = await fetch(`${backendUrl}/health`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
      // Short timeout for readiness check
      signal: AbortSignal.timeout(3000),
    })
    
    const backendHealth = await response.json()
    const isBackendHealthy = response.ok && backendHealth.status === 'ok'
    
    return NextResponse.json({
      ready: isBackendHealthy,
      timestamp: new Date().toISOString(),
      backend: {
        healthy: isBackendHealthy,
        url: backendUrl.replace(/\/+$/, ''),
        response: backendHealth,
      },
    }, {
      status: isBackendHealthy ? 200 : 503
    })
  } catch (error) {
    return NextResponse.json(
      {
        ready: false,
        timestamp: new Date().toISOString(),
        backend: {
          healthy: false,
          error: error instanceof Error ? error.message : 'Unknown error',
        },
      },
      { status: 503 }
    )
  }
}