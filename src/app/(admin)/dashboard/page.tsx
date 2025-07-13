// Updated Dashboard with real DB structure
import StatsCard from "@/components/stats-card";
import { Trophy, Users, Globe, UserCheck } from "lucide-react";
import QuickActions from "@/components/quick-actions";
import RecentActivity from "@/components/recent-activity";

function Dashboard() {	
  return (    
      <div className="py-6">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          {/* Stats Grid */}
          <div className="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-4 mb-8">
            <StatsCard
              title="Countries"
              value="15"
              subtitle="Including Czech Republic, Slovakia, Finland..."
              icon={Globe}
              color="blue"
            />
            <StatsCard
              title="Active Events"
              value="8"
              subtitle="Extraliga, Champions League, World Championship..."
              icon={Trophy}
              color="green"
            />
            <StatsCard
              title="Current Season Teams"
              value="67"
              subtitle="Across all tracked events"
              icon={Users}
              color="purple"
            />
            <StatsCard
              title="Players Tracked"
              value="1,247"
              subtitle="With contract history"
              icon={UserCheck}
              color="yellow"
            />
          </div>

          {/* Quick Actions */}
          <div className="mb-8">
            <QuickActions />
          </div>

          {/* Main Content Grid */}
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
            {/* Recent Activity */}
            <RecentActivity />
            
            {/* Current Season Overview */}
            <div className="bg-white shadow rounded-lg">
              <div className="px-4 py-5 sm:p-6">
                <h3 className="text-lg leading-6 font-medium text-gray-900 mb-4">Current Season Overview</h3>
                <div className="space-y-4">
                  <div className="border-l-4 border-blue-500 pl-4">
                    <div className="flex justify-between items-start">
                      <div>
                        <p className="font-medium text-gray-900">Extraliga 2024/25</p>
                        <p className="text-sm text-gray-500">Czech Republic • 14 teams</p>
                        <p className="text-xs text-gray-400 mt-1">Sponsor: Tipsport</p>
                      </div>
                      <span className="bg-green-100 text-green-800 text-xs px-2 py-1 rounded-full">Active</span>
                    </div>
                  </div>
                  
                  <div className="border-l-4 border-green-500 pl-4">
                    <div className="flex justify-between items-start">
                      <div>
                        <p className="font-medium text-gray-900">Champions League 2024/25</p>
                        <p className="text-sm text-gray-500">International • 32 teams</p>
                      </div>
                      <span className="bg-blue-100 text-blue-800 text-xs px-2 py-1 rounded-full">Group Stage</span>
                    </div>
                  </div>
                  
                  <div className="border-l-4 border-purple-500 pl-4">
                    <div className="flex justify-between items-start">
                      <div>
                        <p className="font-medium text-gray-900">World Championship 2025</p>
                        <p className="text-sm text-gray-500">International • 16 national teams</p>
                      </div>
                      <span className="bg-gray-100 text-gray-800 text-xs px-2 py-1 rounded-full">Upcoming</span>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>

          {/* Team Participation Summary */}
          <div className="mt-8 bg-white shadow rounded-lg">
            <div className="px-4 py-5 sm:p-6">
              <h3 className="text-lg leading-6 font-medium text-gray-900 mb-4">Recent Team Changes</h3>
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                <div className="bg-gray-50 rounded-lg p-4">
                  <div className="flex items-center justify-between mb-2">
                    <h4 className="font-medium text-gray-900">HC Sparta Praha</h4>
                    <span className="text-xs text-gray-500">Czech Republic</span>
                  </div>
                  <p className="text-sm text-gray-600">Participating in Extraliga 2024/25</p>
                  <p className="text-xs text-gray-500 mt-1">25 active player contracts</p>
                </div>
                
                <div className="bg-gray-50 rounded-lg p-4">
                  <div className="flex items-center justify-between mb-2">
                    <h4 className="font-medium text-gray-900">Team Finland</h4>
                    <span className="text-xs text-gray-500">Finland</span>
                  </div>
                  <p className="text-sm text-gray-600">World Championship 2025</p>
                  <p className="text-xs text-gray-500 mt-1">23 selected players</p>
                </div>
                
                <div className="bg-gray-50 rounded-lg p-4">
                  <div className="flex items-center justify-between mb-2">
                    <h4 className="font-medium text-gray-900">Färjestad BK</h4>
                    <span className="text-xs text-gray-500">Sweden</span>
                  </div>
                  <p className="text-sm text-gray-600">Champions League 2024/25</p>
                  <p className="text-xs text-gray-500 mt-1">22 active player contracts</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>    
  );
}

export default Dashboard;