#![deny(warnings)]
use rasterize::*;
use std::sync::Arc;
use tracing_subscriber::fmt::format::FmtSpan;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn firefox(linear: bool) -> Result<Scene> {
    Ok(Scene::group(vec![
        Scene::fill(
            Arc::new("M770.28,91.56c-23.95,27.88-35.1,90.64-10.82,154.26s61.5,49.8,84.7,114.67c30.62,85.6,16.37,200.59,16.37,200.59s36.81,106.61,62.47-6.63C979.79,341.74,770.28,143.94,770.28,91.56Z".parse()?),
            Arc::new(GradRadial::new(
                vec![
                    GradStop::new(0.1, "#ffea00".parse()?),
                    GradStop::new(0.17, "#ffde00".parse()?),
                    GradStop::new(0.28, "#ffbf00".parse()?),
                    GradStop::new(0.43, "#ff8e00".parse()?),
                    GradStop::new(0.77, "#ff272d".parse()?),
                    GradStop::new(0.87, "#e0255a".parse()?),
                    GradStop::new(0.95, "#cc2477".parse()?),
                    GradStop::new(1.0, "#c42482".parse()?),
                ],
                Units::UserSpaceOnUse,
                linear,
                GradSpread::default(),
                Transform::new(0.76, 0.05, 11485.79, 0.03, -1.12, 11147.95),
                (-14491.11, 9293.72),
                450.88,
                (-14519.88, 9293.72),
                0.0,
            )),
            FillRule::default(),
        ),
        Scene::fill(
            Arc::new("M478.07,974.64c245.24,0,443.9-199.74,443.9-446s-198.66-446-443.66-446S34.65,282.32,34.65,528.61C34.18,775.14,233.07,974.64,478.07,974.64Z".parse()?),
            Arc::new(GradRadial::new(
                vec![
                    GradStop::new(0.0, "#00ccda".parse()?),
                    GradStop::new(0.22, "#0083ff".parse()?),
                    GradStop::new(0.26, "#007af9".parse()?),
                    GradStop::new(0.33, "#0060e8".parse()?),
                    GradStop::new(0.33, "#005fe7".parse()?),
                    GradStop::new(0.44, "#2639ad".parse()?),
                    GradStop::new(0.52, "#401e84".parse()?),
                    GradStop::new(0.57, "#4a1475".parse()?),
                ],
                Units::UserSpaceOnUse,
                linear,
                GradSpread::default(),
                Transform::new(1.23, 0.0, 9957.91, 0.0, -1.23,  11055.45),
                (-7587.48, 8863.48),
                791.23,
                (-7587.48, 8863.48),
                0.0,
            )),
            FillRule::default(),
        ),
        Scene::fill(
            Arc::new("M810.67,803.64a246.8,246.8,0,0,1-30.12,18.18,705.31,705.31,0,0,0,38.3-63c9.46-10.47,18.13-20.65,25.19-31.65,3.44-5.41,7.31-12.08,11.42-19.82,24.92-44.9,52.4-117.56,53.18-192.2v-5.66a257.25,257.25,0,0,0-5.71-55.75c.2,1.43.38,2.86.56,4.29-.22-1.1-.41-2.21-.64-3.31.37,2,.66,4,1,6,5.09,43.22,1.47,85.37-16.68,116.45-.29.45-.58.88-.87,1.32,9.41-47.23,12.56-99.39,2.09-151.6,0,0-4.19-25.38-35.38-102.44-18-44.35-49.83-80.72-78-107.21-24.69-30.55-47.11-51-59.47-64.06C689.72,126,678.9,105.61,674.45,92.31c-3.85-1.93-53.14-49.81-57.05-51.63-21.51,33.35-89.16,137.67-57,235.15,14.58,44.17,51.47,90,90.07,115.74,1.69,1.94,23,25,33.09,77.16,10.45,53.85,5,95.86-16.54,158C641.73,681.24,577,735.12,516.3,740.63c-129.67,11.78-177.15-65.11-177.15-65.11C385.49,694,436.72,690.17,467.87,671c31.4-19.43,50.39-33.83,65.81-28.15C548.86,648.43,561,632,550.1,615a78.5,78.5,0,0,0-79.4-34.57c-31.43,5.11-60.23,30-101.41,5.89a86.29,86.29,0,0,1-7.73-5.06c-2.71-1.79,8.83,2.72,6.13.69-8-4.35-22.2-13.84-25.88-17.22-.61-.56,6.22,2.18,5.61,1.62-38.51-31.71-33.7-53.13-32.49-66.57,1-10.75,8-24.52,19.75-30.11,5.69,3.11,9.24,5.48,9.24,5.48s-2.43-5-3.74-7.58c.46-.2.9-.15,1.36-.34,4.66,2.25,15,8.1,20.41,11.67,7.07,5,9.33,9.44,9.33,9.44s1.86-1,.48-5.37c-.5-1.78-2.65-7.45-9.65-13.17h.44A81.61,81.61,0,0,1,374.42,478c2-7.18,5.53-14.68,4.75-28.09-.48-9.43-.26-11.87-1.92-15.51-1.49-3.13.83-4.35,3.42-1.1a32.5,32.5,0,0,0-2.21-7.4v-.24c3.23-11.24,68.25-40.46,73-43.88A67.2,67.2,0,0,0,470.59,361c3.62-5.76,6.34-13.85,7-26.11.36-8.84-3.76-14.73-69.51-21.62-18-1.77-28.53-14.8-34.53-26.82-1.09-2.59-2.21-4.94-3.33-7.28a57.68,57.68,0,0,1-2.56-8.43c10.75-30.87,28.81-57,55.37-76.7,1.45-1.32-5.78.34-4.34-1,1.69-1.54,12.71-6,14.79-7,2.54-1.2-10.88-6.9-22.73-5.51-12.07,1.36-14.63,2.8-21.07,5.53,2.67-2.66,11.17-6.15,9.18-6.13-13,2-29.18,9.56-43,18.12a10.66,10.66,0,0,1,.83-4.35c-6.44,2.73-22.26,13.79-26.87,23.14a44.29,44.29,0,0,0,.27-5.4,84.17,84.17,0,0,0-13.19,13.82l-.24.22c-37.36-15-70.23-16-98.05-9.28-6.09-6.11-9.06-1.64-22.91-32.07-.94-1.83.72,1.81,0,0-2.28-5.9,1.39,7.87,0,0-23.28,18.37-53.92,39.19-68.63,53.89-.18.59,17.16-4.9,0,0-6,1.72-5.6,5.28-6.51,37.5-.22,2.44,0,5.18-.22,7.38-11.75,15-19.75,27.64-22.78,34.21-15.19,26.18-31.93,67-48.15,131.55A334.82,334.82,0,0,1,75.2,398.36C61.71,432.63,48.67,486.44,46.07,569.3A482.08,482.08,0,0,1,58.6,518.64,473,473,0,0,0,93.33,719.71c9.33,22.82,24.76,57.46,51,95.4C226.9,902,343.31,956,472.21,956,606.79,956,727.64,897.13,810.67,803.64Z".parse()?),
            Arc::new(GradLinear::new(
                vec![
                    GradStop::new(0.0, "#000f4366".parse()?),
                    GradStop::new(0.48,"#0019622b".parse()?),
                    GradStop::new(1.0, "#00207900".parse()?),
                ],
                Units::UserSpaceOnUse,
                linear,
                GradSpread::default(),
                Transform::new(1.0, 0.0, 0.0, 0.0, -1.0, 984.0),
                (540.64, 254.8),
                (349.2, 881.03),
            )),
            FillRule::default(),
        ),
        Scene::fill(
            Arc::new("M711.1,866.71c162.87-18.86,235-186.7,142.38-190C769.85,674,634,875.61,711.1,866.71Z".parse()?),
            Arc::new(GradRadial::new(
                vec![
                    GradStop::new(0.0, "#ffea00".parse()?),
                    GradStop::new(0.5, "#ff272d".parse()?),
                    GradStop::new(1.0, "#c42482".parse()?),
                ],
                Units::UserSpaceOnUse,
                linear,
                GradSpread::default(),
                Transform::new(1.22, 0.12, 10240.78, 0.12, -1.22,  10765.16),
                (-8337.74, 7467.89),
                266.89,
                (-8337.74, 7467.89),
                0.0,
            )),
            FillRule::default(),
        ),
        Scene::fill(
            Arc::new("M865.21,642.42C977.26,577.21,948,436.34,948,436.34s-43.25,50.24-72.62,130.32C846.4,646,797.84,681.81,865.21,642.42Z".parse()?),
            Arc::new(GradRadial::new(
                vec![
                    GradStop::new(0.0, "#ffe900".parse()?),
                    GradStop::new(0.16, "#ffaf0e".parse()?),
                    GradStop::new(0.32, "#ff7a1b".parse()?),
                    GradStop::new(0.47, "#ff4e26".parse()?),
                    GradStop::new(0.62, "#ff2c2e".parse()?),
                    GradStop::new(0.76, "#ff1434".parse()?),
                    GradStop::new(0.89, "#ff0538".parse()?),
                    GradStop::new(1.0, "#ff0039".parse()?),
                ],
                Units::UserSpaceOnUse,
                linear,
                GradSpread::default(),
                Transform::new(1.22, 0.12, 10240.78, 0.12, -1.22,  10765.16),
                (-8361.89, 7723.8),
                445.68,
                (-8361.89, 7723.8),
                0.0,
            )),
            FillRule::default(),
        ),
        Scene::fill(
            Arc::new("M509.47,950.06C665.7,999.91,800,876.84,717.21,835.74,642,798.68,435.32,926.49,509.47,950.06Z".parse()?),
            Arc::new(GradRadial::new(
                vec![
                    GradStop::new(0.0, "#ff272d".parse()?),
                    GradStop::new(0.5, "#c42482".parse()?),
                    GradStop::new(0.99, "#620700".parse()?),
                ],
                Units::UserSpaceOnUse,
                linear,
                GradSpread::default(),
                Transform::new(1.22, 0.12, 10240.78, 0.12, -1.22,  10765.16),
                (-8298.85, 7310.66),
                408.96,
                (-8298.85, 7310.66),
                0.0,
            )),
            FillRule::default(),
        ),
        Scene::fill(
            Arc::new("M876.85,702.23c3.8-5.36,8.94-22.53,13.48-30.21,27.58-44.52,27.78-80,27.78-80.84,16.66-83.22,15.15-117.2,4.9-180-8.25-50.6-44.32-123.09-75.57-158-32.2-36-9.51-24.25-40.69-50.52-27.33-30.29-53.82-60.29-68.25-72.36C634.22,43.09,636.57,24.58,638.58,21.42c-.34.37-.84.92-1.47,1.64C635.87,18.14,635,14,635,14s-57,57-69,152c-7.83,62,15.38,126.68,49,168a381.62,381.62,0,0,0,59,58h0c25.4,36.48,39.38,81.49,39.38,129.91,0,121.24-98.34,219.53-219.65,219.53a220.14,220.14,0,0,1-49.13-5.52c-57.24-10.92-90.3-39.8-106.78-59.41-9.45-11.23-13.46-19.42-13.46-19.42,51.28,18.37,108,14.53,142.47-4.52,34.75-19.26,55.77-33.55,72.84-27.92,16.82,5.61,30.21-10.67,18.2-27.54-11.77-16.85-42.4-41-87.88-34.29-34.79,5.07-66.66,29.76-112.24,5.84a97.34,97.34,0,0,1-8.55-5c-3-1.77,9.77,2.69,6.79.68-8.87-4.32-24.57-13.73-28.64-17.07-.68-.56,6.88,2.16,6.2,1.6-42.62-31.45-37.3-52.69-36-66,1.07-10.66,8.81-24.32,21.86-29.86,6.3,3.08,10.23,5.43,10.23,5.43s-2.69-4.92-4.14-7.51c.51-.19,1-.15,1.5-.34,5.16,2.23,16.58,8,22.59,11.57,7.83,4.95,10.32,9.36,10.32,9.36s2.06-1,.54-5.33c-.56-1.77-2.93-7.39-10.68-13.07h.48a91.65,91.65,0,0,1,13.13,8.17c2.19-7.12,6.12-14.56,5.25-27.86-.53-9.35-.28-11.78-2.12-15.39-1.65-3.1.92-4.31,3.78-1.09a29.73,29.73,0,0,0-2.44-7.34v-.24c3.57-11.14,75.53-40.12,80.77-43.51a70.24,70.24,0,0,0,21.17-20.63c4-5.72,7-13.73,7.75-25.89.25-5.48-1.44-9.82-20.5-14-11.44-2.49-29.14-4.91-56.43-7.47-19.9-1.76-31.58-14.68-38.21-26.6-1.21-2.57-2.45-4.9-3.68-7.22a53.41,53.41,0,0,1-2.83-8.36,158.47,158.47,0,0,1,61.28-76.06c1.6-1.31-6.4.33-4.8-1,1.87-1.52,14.06-5.93,16.37-6.92,2.81-1.19-12-6.84-25.16-5.47-13.36,1.35-16.19,2.78-23.32,5.49,3-2.64,12.37-6.1,10.16-6.08-14.4,2-32.3,9.48-47.6,18a9.72,9.72,0,0,1,.92-4.31c-7.13,2.71-24.64,13.67-29.73,23a39.79,39.79,0,0,0,.29-5.35,88.55,88.55,0,0,0-14.6,13.7l-.27.22C258.14,196,221.75,195,191,201.72c-6.74-6.06-17.57-15.23-32.89-45.4-1-1.82-1.6,3.75-2.4,2-6-13.81-9.55-36.44-9-52,0,0-12.32,5.61-22.51,29.06-1.89,4.21-3.11,6.54-4.32,8.87-.56.68,1.27-7.7,1-7.24-1.77,3-6.36,7.19-8.37,12.62-1.38,4-3.32,6.27-4.56,11.29l-.29.46c-.1-1.48.37-6.08,0-5.14A235.4,235.4,0,0,0,95.34,186c-5.49,18-11.88,42.61-12.89,74.57-.24,2.42,0,5.14-.25,7.32-13,14.83-21.86,27.39-25.2,33.91-16.81,26-35.33,66.44-53.29,130.46a319.35,319.35,0,0,1,28.54-50C17.32,416.25,2.89,469.62,0,551.8a436.92,436.92,0,0,1,13.87-50.24C11.29,556.36,17.68,624.3,52.32,701c20.57,45,67.92,136.6,183.62,208h0s39.36,29.3,107,51.26c5,1.81,10.06,3.6,15.23,5.33q-2.43-1-4.71-2A484.9,484.9,0,0,0,492.27,984c175.18.15,226.85-70.2,226.85-70.2l-.51.38q3.71-3.49,7.14-7.26c-27.64,26.08-90.75,27.84-114.3,26,40.22-11.81,66.69-21.81,118.17-41.52q9-3.36,18.48-7.64l2-.94c1.25-.58,2.49-1.13,3.75-1.74a349.3,349.3,0,0,0,70.26-44c51.7-41.3,63-81.56,68.83-108.1-.82,2.54-3.37,8.47-5.17,12.32-13.31,28.48-42.84,46-74.91,61a689.05,689.05,0,0,0,42.38-62.44C865.77,729.39,869,713.15,876.85,702.23Z".parse()?),
            Arc::new(GradRadial::new(
                vec![
                    GradStop::new(0.16, "#ffea00".parse()?),
                    GradStop::new(0.23, "#ffde00".parse()?),
                    GradStop::new(0.37, "#ffbf00".parse()?),
                    GradStop::new(0.54, "#ff8e00".parse()?),
                    GradStop::new(0.76, "#ff272d".parse()?),
                    GradStop::new(0.8, "#f92433".parse()?),
                    GradStop::new(0.84, "#e91c45".parse()?),
                    GradStop::new(0.89, "#cf0e62".parse()?),
                    GradStop::new(0.94, "#b5007f".parse()?),
                ],
                Units::UserSpaceOnUse,
                linear,
                GradSpread::default(),
                Transform::new(1.0, 0.0, 0.0, 0.0, -1.0, 984.0),
                (715.19, 589.96),
                782.18,
                (743.17, 576.13),
                0.0,
            )),
            FillRule::default(),
        ),
        Scene::fill(
            Arc::new("M813.92,801c21.08-23.24,40-49.82,54.35-80,36.9-77.58,94-206.58,49-341.31C881.77,273.22,833,215,771.11,158.12,670.56,65.76,642.48,24.52,642.48,0c0,0-116.09,129.41-65.74,264.38s153.46,130,221.68,270.87c80.27,165.74-64.95,346.61-185,397.24,7.35-1.63,267-60.38,280.61-208.88C893.68,726.34,887.83,767.41,813.92,801Z".parse()?),
            Arc::new(GradRadial::new(
                vec![
                    GradStop::new(0.28, "#ffea00".parse()?),
                    GradStop::new(0.4, "#ffdd00".parse()?),
                    GradStop::new(0.63, "#ffba00".parse()?),
                    GradStop::new(0.86, "#ff9100".parse()?),
                    GradStop::new(0.93, "#ff6711".parse()?),
                    GradStop::new(0.99, "#ff4a1d".parse()?),
                ],
                Units::UserSpaceOnUse,
                linear,
                GradSpread::default(),
                Transform::new(1.0, 0.0, 0.0, 0.0, -1.0, 984.0),
                (670.34, 952.71),
                891.45,
                (670.34, 952.71),
                0.0,
            )),
            FillRule::default(),
        ),
        Scene::fill(
            Arc::new("M477.59,319.37c.39-8.77-4.16-14.66-76.68-21.46-29.84-2.76-41.26-30.33-44.75-41.94-10.61,27.56-15,56.49-12.64,91.48,1.61,22.92,17,47.52,24.37,62,0,0,1.64-2.13,2.39-2.91,13.86-14.43,71.94-36.42,77.39-39.54C453.69,363.16,476.58,346.44,477.59,319.37Z".parse()?),
            Arc::new(GradLinear::new(
                vec![
                    GradStop::new(0.0, "#c4248280".parse()?),
                    GradStop::new(0.47, "#ff272d80".parse()?),
                    GradStop::new(0.49, "#ff2c2c82".parse()?),
                    GradStop::new(0.68, "#ff7a1ab8".parse()?),
                    GradStop::new(0.83, "#ffb20dde".parse()?),
                    GradStop::new(0.94, "#ffd605f5".parse()?),
                    GradStop::new(1.0, "#ffe302".parse()?),
                ],
                Units::UserSpaceOnUse,
                linear,
                GradSpread::default(),
                Transform::new(1.22, 0.12, 10240.78, 0.12, -1.22, 10765.16),
                (-9023.45, 7636.92),
                (-8716.42, 7715.18),
            )),
            FillRule::default(),
        ),
        Scene::fill(
            Arc::new("M158.31,156.47c-1-1.82-1.6,3.75-2.4,2-6-13.81-9.58-36.2-8.72-52,0,0-12.32,5.61-22.51,29.06-1.89,4.21-3.11,6.54-4.32,8.86-.56.68,1.27-7.7,1-7.24-1.77,3-6.36,7.19-8.35,12.38-1.65,4.24-3.35,6.52-4.61,11.77-.39,1.43.39-6.32,0-5.38C84.72,201.68,80.19,271,82.69,268,133.17,214.14,191,201.36,191,201.36c-6.15-4.53-19.53-17.63-32.7-44.89Z".parse()?),
            Arc::new(GradLinear::new(
                vec![
                    GradStop::new(0.0, "#89155199".parse()?),
                    GradStop::new(1.0, "#c4248200".parse()?),
                ],
                Units::UserSpaceOnUse,
                linear,
                GradSpread::default(),
                Transform::new(0.99, 0.1, -250.09, 0.1, -0.99, 2306.15),
                (188.87, 2081.23),
                (134.3, 2221.08),
            )),
            FillRule::default(),
        ),
        Scene::fill(
            Arc::new("M349.84,720.1c-69.72-29.77-149-71.75-146-167.14C207.92,427.35,321,452.18,321,452.18c-4.27,1-15.68,9.16-19.72,17.82-4.27,10.83-12.07,35.28,11.55,60.9,37.09,40.19-76.2,95.36,98.66,199.57,4.41,2.4-41-1.43-61.64-10.36Z".parse()?),
            Arc::new(GradLinear::new(
                vec![
                    GradStop::new(0.01, "#89155180".parse()?),
                    GradStop::new(0.48, "#ff272d80".parse()?),
                    GradStop::new(1.0,  "#ff272d00".parse()?),
                ],
                Units::UserSpaceOnUse,
                linear,
                GradSpread::default(),
                Transform::new(0.99, 0.1, 229.04, 0.1, -0.99 , 745.74),
                (-38.43, 278.04),
                (55.68, 171.16),
            )),
            FillRule::default(),
        ),
        Scene::fill(
            Arc::new("M325.07,657.5c49.44,17.21,107,14.19,141.52-4.86,23.09-12.85,52.7-33.43,70.92-28.35-15.78-6.24-27.73-9.15-42.1-9.86-2.45,0-5.38,0-8-.32a136,136,0,0,0-15.76.86c-8.9.82-18.77,6.43-27.74,5.53-.48,0,8.7-3.77,8-3.61-4.75,1-9.92,1.21-15.37,1.88-3.47.39-6.45.82-9.89,1-103,8.73-190-55.81-190-55.81-7.41,25,33.17,74.3,88.52,93.57Z".parse()?),
            Arc::new(GradLinear::new(
                vec![
                    GradStop::new(0.0, "#c42482".parse()?),
                    GradStop::new(0.08, "#c42482cf".parse()?),
                    GradStop::new(0.21, "#c4248291".parse()?),
                    GradStop::new(0.33, "#c424825c".parse()?),
                    GradStop::new(0.45, "#c4248233".parse()?),
                    GradStop::new(0.56, "#c4248217".parse()?),
                    GradStop::new(0.67, "#c4248205".parse()?),
                    GradStop::new(0.77, "#c4248200".parse()?),
                ],
                Units::UserSpaceOnUse,
                linear,
                GradSpread::default(),
                Transform::new(0.99, 0.1, 229.04, 0.1, -0.99 , 745.74),
                (142.46, 93.68),
                (142.53, 168.46),
            )),
            FillRule::default(),
        ),
        Scene::fill(
            Arc::new("M813.74,801.65c104.16-102.27,156.86-226.58,134.58-366,0,0,8.9,71.5-24.85,144.63,16.21-71.39,18.1-160.11-25-252C841,205.64,746.45,141.11,710.35,114.19,655.66,73.4,633,31.87,632.57,23.3c-16.34,33.48-65.77,148.2-5.31,247,56.64,92.56,145.86,120,208.33,205C950.67,631.67,813.74,801.65,813.74,801.65Z".parse()?),
            Arc::new(GradLinear::new(
                vec![
                    GradStop::new(0.0, "#fff14f".parse()?),
                    GradStop::new(0.27, "#ffee4c".parse()?),
                    GradStop::new(0.45, "#ffe643".parse()?),
                    GradStop::new(0.61, "#ffd834".parse()?),
                    GradStop::new(0.76, "#ffc41e".parse()?),
                    GradStop::new(0.89, "#ffab02".parse()?),
                    GradStop::new(0.9, "#ffa900".parse()?),
                    GradStop::new(0.95, "#ffa000".parse()?),
                    GradStop::new(1.0, "#ff9100".parse()?),
                ],
                Units::UserSpaceOnUse,
                linear,
                GradSpread::default(),
                Transform::new(1.0, 0.0, 0.0, 0.0, -1.0, 984.0),
                (620.52, 947.88),
                (926.18, 264.39),
            )),
            FillRule::default(),
        ),
        Scene::fill(
            Arc::new("M798.81,535.55C762.41,460.35,717,427.55,674,392c5,7,6.23,9.47,9,14,37.83,40.32,93.61,138.66,53.11,262.11C659.88,900.48,355,791.06,323,760.32,335.93,894.81,561,959.16,707.6,872,791,793,858.47,658.79,798.81,535.55Z".parse()?),
            Arc::new(GradLinear::new(
                vec![
                    GradStop::new(0.0, "#ff8e00".parse()?),
                    GradStop::new(0.04, "#ff8e00db".parse()?),
                    GradStop::new(0.08, "#ff8e00ba".parse()?),
                    GradStop::new(0.13, "#ff8e00a0".parse()?),
                    GradStop::new(0.18, "#ff8e008e".parse()?),
                    GradStop::new(0.23, "#ff8e0082".parse()?),
                    GradStop::new(0.28, "#ff8e007f".parse()?),
                    GradStop::new(0.39, "#ff8e007a".parse()?),
                    GradStop::new(0.52, "#ff8e006b".parse()?),
                    GradStop::new(0.68, "#ff8e004f".parse()?),
                    GradStop::new(0.84, "#ff8e002b".parse()?),
                    GradStop::new(1.0, "#ff8e0000".parse()?),
                ],
                Units::UserSpaceOnUse,
                linear,
                GradSpread::default(),
                Transform::new(1.0, 0.0, 0.0, 0.0, -1.0, 984.0),
                (680.88, 554.79),
                (536.1, 166.04),
            )),
            FillRule::default(),
        ),
    ]))
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("debug")
        .with_span_events(FmtSpan::CLOSE)
        .with_writer(std::io::stderr)
        .init();

    let scene = firefox(false)?;

    let view = BBox::new((0.0, 0.0), (953.37, 984.0));
    let size = BBox::new((0.0, 0.0), (512.0, 512.0));
    let tr = Transform::fit_bbox(view, size, Align::Mid);

    let rasterizer = ActiveEdgeRasterizer::default();

    let span = tracing::info_span!("render");
    let img = span.in_scope(|| {
        let img = scene.render_pipeline(&rasterizer, tr, Some(size), Some("#00000000".parse()?));
        Result::Ok(img)
    })?;
    img.write_bmp(std::io::stdout())?;

    Ok(())
}
