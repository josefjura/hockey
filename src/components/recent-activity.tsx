// components/recent-activity.tsx
import { Globe, Trophy, Users, UserPlus } from "lucide-react";

function RecentActivity() {
  const activities = [
    { 
      id: 1, 
      action: 'Player contract added', 
      details: 'David Pastrňák signed with HC Sparta Praha (Extraliga 2024/25)', 
      time: '2 hours ago',
      icon: UserPlus,
      color: 'bg-green-500'
    },
    { 
      id: 2, 
      action: 'New season created', 
      details: 'Champions League 2024/25 season initialized', 
      time: '1 day ago',
      icon: Trophy,
      color: 'bg-blue-500'
    },
    { 
      id: 3, 
      action: 'Team participation', 
      details: 'Färjestad BK added to Champions League 2024/25', 
      time: '2 days ago',
      icon: Users,
      color: 'bg-purple-500'
    },
    { 
      id: 4, 
      action: 'Country added', 
      details: 'Norway added to country database', 
      time: '3 days ago',
      icon: Globe,
      color: 'bg-orange-500'
    },
  ];

  return (
    <div className="bg-white shadow rounded-lg">
      <div className="px-4 py-5 sm:p-6">
        <h3 className="text-lg leading-6 font-medium text-gray-900 mb-4">Recent Activity</h3>
        <div className="flow-root">
          <ul className="-mb-8">
            {activities.map((activity, activityIdx) => (
              <li key={activity.id}>
                <div className="relative pb-8">
                  {activityIdx !== activities.length - 1 ? (
                    <span className="absolute top-4 left-4 -ml-px h-full w-0.5 bg-gray-200" />
                  ) : null}
                  <div className="relative flex space-x-3">
                    <div>
                      <span className={`${activity.color} h-8 w-8 rounded-full flex items-center justify-center ring-8 ring-white`}>
                        <activity.icon className="h-4 w-4 text-white" />
                      </span>
                    </div>
                    <div className="min-w-0 flex-1 pt-1.5 flex justify-between space-x-4">
                      <div>
                        <p className="text-sm text-gray-900">
                          <span className="font-medium">{activity.action}</span> - {activity.details}
                        </p>
                      </div>
                      <div className="text-right text-sm whitespace-nowrap text-gray-500">
                        {activity.time}
                      </div>
                    </div>
                  </div>
                </div>
              </li>
            ))}
          </ul>
        </div>
      </div>
    </div>
  );
}

export default RecentActivity;