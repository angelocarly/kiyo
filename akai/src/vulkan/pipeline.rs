use ash::vk;

pub trait Pipeline {
    fn handle(&self) -> vk::Pipeline;
    fn bind_point(&self) -> vk::PipelineBindPoint;
}
