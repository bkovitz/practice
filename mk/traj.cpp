/*
 * traj.cpp -- A little exercise in multithreading invented by Grok.
 */

#include <iostream>
#include <vector>
#include <thread>
#include <mutex>
#include <condition_variable>
#include <cmath>
#include <algorithm>
#include <chrono>

struct TrajectorySegment {
  double start_x, start_y;
  double end_x, end_y;
};

class TrajectoryProcessor {
  std::vector<TrajectorySegment> segments;
  double total_cost = 0.0;
  std::mutex mtx;
  std::condition_variable cv;
  size_t tasks_completed = 0;

public:
  TrajectoryProcessor(const std::vector<TrajectorySegment>& segs)
  : segments(segs) { }

  static double compute_cost(const TrajectorySegment& seg) {
    return std::sqrt(std::pow(seg.end_x - seg.start_x, 2)
                     +
                     std::pow(seg.end_y - seg.start_y, 2));
  }

  void process_range(size_t start_idx, size_t end_idx) {
    // start_idx inclusive, end_idx exclusive
    std::string me = (std::ostringstream{} << "(" << start_idx << ", " << end_idx << ")").str();
    //std::cout << me << " start" << std::endl;
    double local_cost = 0.0;
    for (size_t i = start_idx; i < end_idx; i++)
      local_cost += compute_cost(segments[i]);
    {
      std::lock_guard lock{mtx};
      //std::cout << me << " local_cost: " << local_cost << std::endl;
      //std::cout << me << " before: " << total_cost << std::endl;
      total_cost += local_cost;
      //std::this_thread::sleep_for(std::chrono::milliseconds(1000));
      //std::cout << me << " after:  " << total_cost << std::endl;
    }
    // TODO signal completion of this thread's task
  }

  void process(size_t num_threads) {
    std::vector<std::thread> threads;
    size_t segments_per_thread = std::max(
      size_t{1},
      (segments.size() + num_threads - 1) / num_threads
    );

    // Launch threads, assigning each a range of segments
    size_t start_idx = 0;
    while (start_idx < segments.size()) {
      size_t end_idx = std::min(
        segments.size(),
        start_idx + segments_per_thread
      );
      threads.emplace_back(
        &TrajectoryProcessor::process_range,
        this,
        start_idx,
        end_idx
      );
      start_idx = end_idx;
    }
    for (auto& thread : threads)
      thread.join();
    // This seems unnecessary: TODO Notify main thread and print total_cost
  }

  double get_total_cost() const { return total_cost; }
};

int main() {
  std::vector<TrajectorySegment> segments = {
    {0.0, 0.0, 1.0, 1.0},  // Segment 1
    {1.0, 1.0, 2.0, 2.0},  // Segment 2
    {2.0, 2.0, 3.0, 3.0},  // Segment 3
    {3.0, 3.0, 4.0, 4.0},  // Segment 4
  };
//  for (int i = 0; i < 1000; i++)
//    segments.push_back({3.0, 3.0, 5.0, 5.0});
  TrajectoryProcessor processor(segments);
  processor.process(2);
  std::cout << "Total trajectory cost: " << processor.get_total_cost()
            << std::endl;
//  double total = 0.0;
//  for (const auto& s : segments)
//    total += TrajectoryProcessor::compute_cost(s);
//  std::cout << "True cost: " << total << std::endl;
  return 0;
}
