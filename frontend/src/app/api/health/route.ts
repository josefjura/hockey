import { NextResponse } from 'next/server'

export async function GET() {
  try {
    // Basic health check - verify the API can respond
    const backendUrl = process.env.HOCKEY_BACKEND_URL || 'http://localhost:8080'
    
    return NextResponse.json({
      status: 'ok',
      timestamp: new Date().toISOString(),
      backend_url: backendUrl.replace(/\/+$/, ''), // Remove trailing slashes
    })
  } catch (error) {
    return NextResponse.json(
      {
        status: 'error',
        timestamp: new Date().toISOString(),
        error: error instanceof Error ? error.message : 'Unknown error',
      },
      { status: 500 }
    )
  }
}