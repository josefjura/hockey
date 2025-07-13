// components/quick-actions.tsx
import { Globe, Trophy, Users, UserPlus } from "lucide-react";

function QuickActions() {
  const actions = [
    { name: 'Add Country', href: '/countries/new', icon: Globe, color: 'bg-blue-600 hover:bg-blue-700' },
    { name: 'Create Event', href: '/events/new', icon: Trophy, color: 'bg-green-600 hover:bg-green-700' },
    { name: 'Add Team', href: '/teams/new', icon: Users, color: 'bg-purple-600 hover:bg-purple-700' },
    { name: 'Register Player', href: '/players/new', icon: UserPlus, color: 'bg-orange-600 hover:bg-orange-700' },
  ];

  return (
    <div className="bg-white shadow rounded-lg">
      <div className="px-4 py-5 sm:p-6">
        <h3 className="text-lg leading-6 font-medium text-gray-900 mb-4">Quick Actions</h3>
        <div className="grid grid-cols-2 gap-3 sm:grid-cols-4">
          {actions.map((action) => (
            <button
              key={action.name}
              className={`${action.color} text-white px-3 py-2 rounded-md text-sm font-medium flex items-center justify-center space-x-2 transition-colors`}
            >
              <action.icon className="h-4 w-4" />
              <span>{action.name}</span>
            </button>
          ))}
        </div>
      </div>
    </div>
  );
}

export default QuickActions;