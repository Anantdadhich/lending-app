"use client"

import { motion } from "framer-motion"


import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"

import { Button } from "@/components/ui/button"
import {
  Github,
  ArrowRight,
  Code2,
  Wallet,
  TrendingUp,
  Shield,
  Zap,
  Star,
  Users,
  DollarSign,
  BarChart3,
} from "lucide-react"
import { WalletButton } from "../solana/solana-provider"
import { Badge } from "../ui/badge"

const containerVariants = {
  hidden: { opacity: 0 },
  visible: {
    opacity: 1,
    transition: {
      staggerChildren: 0.1,
      delayChildren: 0.2,
    },
  },
}



export const LandingPage = ()=> {
  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-50 via-blue-50 to-indigo-100 relative overflow-hidden ">
      {/* Background Elements */}
      <div className="absolute inset-0 overflow-hidden">
        {/* Grid Pattern */}
        <div className="absolute inset-0 bg-[linear-gradient(to_right,#8882_1px,transparent_1px),linear-gradient(to_bottom,#8882_1px,transparent_1px)] bg-[size:14px_24px]"></div>

        {/* Floating Elements */}
        <motion.div
          className="absolute top-20 right-20 w-72 h-72 bg-gradient-to-br from-blue-400/20 to-indigo-600/20 rounded-full blur-3xl"
       
          animate="animate"
        />
        <motion.div
          className="absolute bottom-20 left-20 w-96 h-96 bg-gradient-to-br from-indigo-400/15 to-blue-600/15 rounded-full blur-3xl"
       
          animate="animate"
          style={{ animationDelay: "2s" }}
        />
        <motion.div
          className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-[600px] h-[600px] bg-gradient-to-br from-blue-300/10 to-indigo-500/10 rounded-full blur-3xl"

          animate="animate"
          style={{ animationDelay: "4s" }}
        />
      </div>

   

      {/* Main Content */}
      <motion.main
        className="relative z-10 max-w-7xl mx-auto px-6 py-20"
        variants={containerVariants}
        initial="hidden"
        animate="visible"
      >
        {/* Hero Section */}
        <div className="text-center mb-20">
          <motion.div
            className="inline-flex items-center gap-2 bg-blue-50 border border-blue-200 rounded-full px-4 py-2 mb-8"
          
            whileHover={{ scale: 1.05 }}
            transition={{ type: "spring", stiffness: 300 }}
          >
            <Star className="w-4 h-4 text-blue-600" />
            <span className="text-sm text-blue-700 font-medium">Powered by GitHub Reputation</span>
            <Badge variant="secondary" className="bg-emerald-100 text-emerald-700 border-emerald-200 text-xs">
              Beta
            </Badge>
          </motion.div>

          <motion.h1 className="text-6xl md:text-7xl lg:text-8xl font-bold mb-8 leading-tight" >
            <span className="text-gray-900">Code Your Way to</span>
            <br />
            <span className="bg-gradient-to-r from-blue-600 via-indigo-600 to-blue-800 bg-clip-text text-transparent">
              Instant Credit
            </span>
          </motion.h1>

          <motion.p
            className="text-xl md:text-2xl text-gray-600 max-w-4xl mx-auto leading-relaxed mb-12"
 
          >
            Transform your GitHub contributions into financial opportunities. Get instant USDC loans based on your
            development activity and reputation in the open source community.
          </motion.p>

          <motion.div
            className="flex flex-col sm:flex-row items-center justify-center gap-4 mb-16"
  
          >
            <Button
              size="lg"
              className="bg-gradient-to-r from-blue-600 to-indigo-600 hover:from-blue-700 hover:to-indigo-700 text-white px-8 py-4 text-lg font-semibold rounded-2xl shadow-xl hover:shadow-blue-500/25 transition-all duration-300 group"
            >
              Get Started Now
              <ArrowRight className="w-5 h-5 ml-2 group-hover:translate-x-1 transition-transform" />
            </Button>
            <Button
              variant="outline"
              size="lg"
              className="bg-white/50 border-gray-300 text-gray-700 hover:bg-white hover:border-blue-300 px-8 py-4 text-lg font-semibold rounded-2xl backdrop-blur-sm"
            >
              View Documentation
            </Button>
          </motion.div>

          {/* Stats */}
          <motion.div className="grid grid-cols-2 md:grid-cols-4 gap-8 max-w-4xl mx-auto" >
            {[
              { icon: Users, label: "Active Developers", value: "3,247", color: "text-blue-600" },
              { icon: DollarSign, label: "Total Loans", value: "$2.1M", color: "text-emerald-600" },
              { icon: BarChart3, label: "Success Rate", value: "98.7%", color: "text-indigo-600" },
              { icon: TrendingUp, label: "Avg APR", value: "5.2%", color: "text-purple-600" },
            ].map((stat, index) => (
              <motion.div
                key={stat.label}
                className="text-center group"
                whileHover={{ y: -5, scale: 1.05 }}
                transition={{ type: "spring", stiffness: 300 }}
              >
                <div className="bg-white/70 backdrop-blur-sm border border-gray-200/50 rounded-2xl p-6 shadow-lg hover:shadow-xl transition-all duration-300">
                  <stat.icon className={`w-8 h-8 ${stat.color} mx-auto mb-3`} />
                  <div className="text-2xl font-bold text-gray-900 mb-1">{stat.value}</div>
                  <div className="text-sm text-gray-600">{stat.label}</div>
                </div>
              </motion.div>
            ))}
          </motion.div>
        </div>

        {/* Connection Section */}
        <motion.div className="max-w-5xl mx-auto mb-20" >
          <div className="text-center mb-12">
            <h2 className="text-4xl font-bold text-gray-900 mb-4">Get Started in 2 Simple Steps</h2>
            <p className="text-xl text-gray-600">Connect your accounts and start borrowing within minutes</p>
          </div>

          <div className="grid md:grid-cols-2 gap-8">
            <motion.div
              whileHover={{ y: -8, scale: 1.02 }}
              transition={{ type: "spring", stiffness: 300, damping: 20 }}
            >
              <Card className="bg-white/80 backdrop-blur-md border-gray-200/50 hover:border-blue-300/50 transition-all duration-500 shadow-xl hover:shadow-2xl group">
                <CardHeader className="pb-6">
                  <div className="flex items-center gap-4 mb-4">
                    <motion.div
                      className="w-14 h-14 bg-gradient-to-br from-blue-500 to-indigo-600 rounded-2xl flex items-center justify-center shadow-lg group-hover:shadow-blue-500/25 transition-all duration-300"
                      whileHover={{ rotate: 360 }}
                      transition={{ duration: 0.6 }}
                    >
                      <Wallet className="w-7 h-7 text-white" />
                    </motion.div>
                    <div>
                      <CardTitle className="text-gray-900 text-2xl font-bold">Connect Wallet</CardTitle>
                      <CardDescription className="text-gray-600 text-base">Step 1 • Secure Connection</CardDescription>
                    </div>
                  </div>
                </CardHeader>
                <CardContent className="space-y-6">
                  <p className="text-gray-700 text-lg leading-relaxed">
                    Connect your Solana wallet to securely interact with our lending protocol. We support all major
                    wallets.
                  </p>
                  <WalletButton />
                </CardContent>
              </Card>
            </motion.div>

            <motion.div
              whileHover={{ y: -8, scale: 1.02 }}
              transition={{ type: "spring", stiffness: 300, damping: 20 }}
            >
              <Card className="bg-white/80 backdrop-blur-md border-gray-200/50 hover:border-blue-300/50 transition-all duration-500 shadow-xl hover:shadow-2xl group">
                <CardHeader className="pb-6">
                  <div className="flex items-center gap-4 mb-4">
                    <motion.div
                      className="w-14 h-14 bg-gradient-to-br from-gray-800 to-gray-900 rounded-2xl flex items-center justify-center shadow-lg group-hover:shadow-gray-500/25 transition-all duration-300"
                      whileHover={{ rotate: 360 }}
                      transition={{ duration: 0.6 }}
                    >
                      <Github className="w-7 h-7 text-white" />
                    </motion.div>
                    <div>
                      <CardTitle className="text-gray-900 text-2xl font-bold">Link GitHub</CardTitle>
                      <CardDescription className="text-gray-600 text-base">
                        Step 2 • Reputation Analysis
                      </CardDescription>
                    </div>
                  </div>
                </CardHeader>
                <CardContent className="space-y-6">
                  <p className="text-gray-700 text-lg leading-relaxed">
                    Connect your GitHub account to analyze your development activity and calculate your credit score.
                  </p>
                
                </CardContent>
              </Card>
            </motion.div>
          </div>
        </motion.div>

        {/* Features Section */}
        <motion.div className="max-w-6xl mx-auto" >
          <div className="text-center mb-16">
            <h2 className="text-4xl font-bold text-gray-900 mb-4">How It Works</h2>
            <p className="text-xl text-gray-600 max-w-3xl mx-auto">
              Our algorithm analyzes your GitHub activity to determine your creditworthiness in the developer ecosystem
            </p>
          </div>

          <div className="grid md:grid-cols-3 gap-8">
            {[
              {
                icon: TrendingUp,
                title: "Reputation Analysis",
                description:
                  "We analyze your commits, pull requests, issues, and contributions to calculate a comprehensive developer score.",
                color: "from-emerald-500 to-teal-600",
                bgColor: "bg-emerald-50",
                iconColor: "text-emerald-600",
              },
              {
                icon: Shield,
                title: "Credit Assessment",
                description:
                  "Your reputation score determines loan eligibility: 80+ score gets $100, 50-79 gets $50 USDC.",
                color: "from-blue-500 to-indigo-600",
                bgColor: "bg-blue-50",
                iconColor: "text-blue-600",
              },
              {
                icon: Zap,
                title: "Instant Funding",
                description:
                  "Once approved, receive USDC loans instantly with flexible repayment terms and competitive rates.",
                color: "from-purple-500 to-indigo-600",
                bgColor: "bg-purple-50",
                iconColor: "text-purple-600",
              },
            ].map((feature, index) => (
              <motion.div
                key={feature.title}
                className="group"
              
                whileHover={{ y: -10, scale: 1.02 }}
                transition={{ type: "spring", stiffness: 300 }}
              >
                <Card className="bg-white/80 backdrop-blur-md border-gray-200/50 hover:border-blue-300/50 transition-all duration-500 shadow-xl hover:shadow-2xl h-full">
                  <CardContent className="p-8 text-center">
                    <motion.div
                      className={`w-16 h-16 ${feature.bgColor} rounded-2xl flex items-center justify-center mx-auto mb-6 group-hover:shadow-lg transition-all duration-300`}
                      whileHover={{ scale: 1.1, rotate: 5 }}
                      transition={{ type: "spring", stiffness: 300 }}
                    >
                      <feature.icon className={`w-8 h-8 ${feature.iconColor}`} />
                    </motion.div>
                    <h3 className="text-xl font-bold text-gray-900 mb-4">{feature.title}</h3>
                    <p className="text-gray-600 leading-relaxed">{feature.description}</p>
                  </CardContent>
                </Card>
              </motion.div>
            ))}
          </div>
        </motion.div>
      </motion.main>
    </div>
  )
}
